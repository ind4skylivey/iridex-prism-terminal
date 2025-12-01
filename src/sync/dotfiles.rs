use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use dirs;
use serde_json;
use sha2::{Digest, Sha256};

use crate::ensure_config_dir;
use crate::error::{PrismError, PrismResult};
use crate::metadata_dir;

#[derive(Debug, Clone)]
pub struct DotfileDescriptor {
    pub name: String,
    pub path: PathBuf,
    pub contents: Vec<u8>,
    pub size: u64,
    pub modified: Option<String>,
    pub sha256: String,
    pub permissions: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RestoreOutcome {
    pub overwritten_backup: Option<PathBuf>,
    pub snapshot_path: PathBuf,
}

pub fn tracked_files() -> PrismResult<Vec<PathBuf>> {
    let base = ensure_config_dir()?.join("dotfiles");
    if !base.exists() {
        return Ok(vec![]);
    }
    let exclusions = load_exclusions()?.into_iter().collect::<HashSet<_>>();
    let mut files = Vec::new();
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str().map(|value| value.to_string()) {
                if exclusions.contains(&name) {
                    continue;
                }
            }
            files.push(entry.path());
        }
    }
    Ok(files)
}

pub fn track(path: &Path) -> PrismResult<PathBuf> {
    if !path.exists() {
        return Err(PrismError::new("dotfile does not exist"));
    }
    let dest_dir = ensure_config_dir()?.join("dotfiles");
    fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join(
        path.file_name()
            .ok_or_else(|| PrismError::new("dotfile missing filename"))?,
    );
    fs::copy(path, &dest)?;
    Ok(dest)
}

pub fn describe(path: &Path) -> PrismResult<DotfileDescriptor> {
    if !path.exists() {
        return Err(PrismError::new("dotfile does not exist"));
    }
    let metadata = fs::metadata(path)?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| PrismError::new("dotfile missing filename"))?
        .to_string();
    let contents = fs::read(path)?;
    let sha = hash_contents(&contents);
    let size = metadata.len();
    let modified = metadata.modified().ok().and_then(system_time_to_rfc3339);
    Ok(DotfileDescriptor {
        name,
        path: path.to_path_buf(),
        contents,
        size,
        modified,
        sha256: sha,
        permissions: permission_string(&metadata),
    })
}

pub fn hash_contents(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
}

fn system_time_to_rfc3339(time: SystemTime) -> Option<String> {
    let datetime: DateTime<Utc> = time.into();
    Some(datetime.to_rfc3339())
}

#[cfg(unix)]
fn permission_string(metadata: &fs::Metadata) -> Option<String> {
    use std::os::unix::fs::PermissionsExt;
    Some(format!("{:o}", metadata.permissions().mode()))
}

#[cfg(not(unix))]
fn permission_string(_: &fs::Metadata) -> Option<String> {
    None
}

pub const EXCLUSIONS_FILE: &str = "dotfiles-exclusions.json";

#[derive(Clone, Debug)]
struct ExclusionsCache {
    path: PathBuf,
    modified: Option<SystemTime>,
    len: u64,
    values: Vec<String>,
}

impl ExclusionsCache {
    fn empty() -> Self {
        Self {
            path: PathBuf::new(),
            modified: None,
            len: 0,
            values: Vec::new(),
        }
    }
}

static EXCLUSIONS_CACHE: OnceLock<Mutex<ExclusionsCache>> = OnceLock::new();

fn normalize_exclusions(entries: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut normalized = entries
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    normalized.sort_unstable();
    normalized.dedup();
    normalized
}

pub fn load_exclusions() -> PrismResult<Vec<String>> {
    let path = exclusions_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let metadata = match fs::metadata(&path) {
        Ok(meta) => meta,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(err) => return Err(err.into()),
    };
    let modified = metadata.modified().ok();
    let len = metadata.len();

    if let Some(cache) = EXCLUSIONS_CACHE.get() {
        if let Ok(guard) = cache.lock() {
            if guard.path == path && guard.len == len && guard.modified == modified {
                return Ok(guard.values.clone());
            }
        }
    }

    let raw = fs::read(&path)?;
    let entries: Vec<String> = serde_json::from_slice(&raw).unwrap_or_else(|_| Vec::new());
    let normalized = normalize_exclusions(entries);

    let cache = EXCLUSIONS_CACHE.get_or_init(|| Mutex::new(ExclusionsCache::empty()));
    if let Ok(mut guard) = cache.lock() {
        *guard = ExclusionsCache {
            path: path.clone(),
            modified,
            len,
            values: normalized.clone(),
        };
    }

    Ok(normalized)
}

pub fn save_exclusions(exclusions: &[String]) -> PrismResult<()> {
    let path = exclusions_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let normalized = normalize_exclusions(exclusions.iter().cloned());
    let serialized = serde_json::to_string_pretty(&normalized)?;
    fs::write(&path, serialized)?;

    if let Ok(metadata) = fs::metadata(&path) {
        let modified = metadata.modified().ok();
        let len = metadata.len();
        let cache = EXCLUSIONS_CACHE.get_or_init(|| Mutex::new(ExclusionsCache::empty()));
        if let Ok(mut guard) = cache.lock() {
            *guard = ExclusionsCache {
                path,
                modified,
                len,
                values: normalized,
            };
        }
    }
    Ok(())
}

pub fn add_exclusion(name: &str) -> PrismResult<()> {
    let mut list = load_exclusions()?;
    if list.iter().any(|value| value == name) {
        return Ok(());
    }
    list.push(name.to_string());
    list.sort();
    list.dedup();
    save_exclusions(&list)
}

pub fn remove_exclusion(name: &str) -> PrismResult<()> {
    let mut list = load_exclusions()?;
    list.retain(|value| value != name);
    save_exclusions(&list)
}

fn exclusions_path() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join(EXCLUSIONS_FILE))
}

pub fn dotfiles_root() -> PrismResult<PathBuf> {
    let dir = ensure_config_dir()?.join("dotfiles");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn restore_to(name: &str, target: &Path) -> PrismResult<RestoreOutcome> {
    let source = dotfiles_root()?.join(name);
    if !source.exists() {
        return Err(PrismError::new(format!("dotfile '{name}' not found")));
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    let backup = backup_existing(target)?;
    fs::copy(&source, target)?;
    let snapshot = persist_restore_snapshot(name, &source)?;
    Ok(RestoreOutcome {
        overwritten_backup: backup,
        snapshot_path: snapshot,
    })
}

pub fn default_restore_destination(name: &str) -> PrismResult<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| PrismError::new("missing home directory"))?;
    Ok(home.join(name))
}

fn backup_existing(target: &Path) -> PrismResult<Option<PathBuf>> {
    if !target.exists() {
        return Ok(None);
    }
    let backup_dir = backups_dir()?;
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    let file_name = target
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("dotfile");
    let backup_path = backup_dir.join(format!(
        "{timestamp}-existing-{name}",
        name = sanitize_name(file_name)
    ));
    fs::copy(target, &backup_path)?;
    Ok(Some(backup_path))
}

fn persist_restore_snapshot(name: &str, source: &Path) -> PrismResult<PathBuf> {
    let backup_dir = backups_dir()?;
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    let backup_path = backup_dir.join(format!(
        "{timestamp}-restore-{name}",
        name = sanitize_name(name)
    ));
    fs::copy(source, &backup_path)?;
    Ok(backup_path)
}

fn backups_dir() -> PrismResult<PathBuf> {
    let dir = metadata_dir()?.join("dotfile-backups");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | ' ' => '-',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn describe_returns_hash_and_metadata() {
        let mut file = NamedTempFile::new().expect("temp file");
        write!(file, "hello world").expect("write");
        let info = describe(file.path()).expect("describe");
        assert_eq!(info.size, 11);
        assert_eq!(info.contents, b"hello world");
        assert_eq!(
            info.sha256,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        assert!(info.modified.is_some());
    }
}
