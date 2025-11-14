use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

use crate::ensure_config_dir;
use crate::error::{PrismError, PrismResult};

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

pub fn tracked_files() -> PrismResult<Vec<PathBuf>> {
    let base = ensure_config_dir()?.join("dotfiles");
    if !base.exists() {
        return Ok(vec![]);
    }
    let mut files = Vec::new();
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
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
