use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::core::theme::{Theme, ThemeMetadata};
use crate::error::{PrismError, PrismResult};
use crate::{themes_root, user_themes_dir};

#[derive(Debug, Clone)]
pub struct ThemeCatalogEntry {
    pub metadata: ThemeMetadata,
    pub path: PathBuf,
    pub source: ThemeSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeSource {
    BuiltIn,
    User,
}

pub fn load_theme_by_name(name: &str) -> PrismResult<Theme> {
    let mut lookup = build_lookup()?;
    let path = lookup
        .remove(&name.to_lowercase())
        .ok_or_else(|| PrismError::new(format!("theme '{name}' not found")))?;
    Theme::load(&path)
}

pub fn list_available() -> PrismResult<Vec<ThemeCatalogEntry>> {
    let mut entries = Vec::new();
    for (path, source) in walk_theme_paths()? {
        let theme = Theme::load(&path)?;
        entries.push(ThemeCatalogEntry {
            metadata: theme.metadata,
            path,
            source,
        });
    }
    Ok(entries)
}

fn walk_theme_paths() -> PrismResult<Vec<(PathBuf, ThemeSource)>> {
    let mut paths = Vec::new();
    let builtin = themes_root()?;
    if builtin.exists() {
        push_paths(&builtin, ThemeSource::BuiltIn, &mut paths)?;
    }
    let user = user_themes_dir()?;
    push_paths(&user, ThemeSource::User, &mut paths)?;
    Ok(paths)
}

fn push_paths(
    dir: &Path,
    source: ThemeSource,
    output: &mut Vec<(PathBuf, ThemeSource)>,
) -> PrismResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in WalkDir::new(dir).max_depth(1) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map(|s| s == "toml")
                .unwrap_or(false)
        {
            output.push((entry.path().to_path_buf(), source));
        }
    }
    Ok(())
}

fn build_lookup() -> PrismResult<BTreeMap<String, PathBuf>> {
    let mut map = BTreeMap::new();
    for (path, _) in walk_theme_paths()? {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PrismError::new("invalid theme filename"))?;
        map.insert(name.to_lowercase(), path);
    }
    Ok(map)
}
