use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::core::shared_palette;
use crate::core::theme::Theme;
use crate::error::{PrismError, PrismResult};
use crate::{themes_root, user_themes_dir};

#[derive(Debug, Clone)]
pub struct ThemeCatalogEntry {
    pub theme: Theme,
    pub path: PathBuf,
    pub source: ThemeSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeSource {
    BuiltIn,
    User,
    Palette,
}

pub fn load_theme_by_name(name: &str) -> PrismResult<Theme> {
    let builtin = themes_root()?;
    let user = user_themes_dir()?;
    load_theme_by_name_with_roots(name, &builtin, &user)
}

pub fn load_theme_by_name_with_roots(
    name: &str,
    builtin_root: &Path,
    user_root: &Path,
) -> PrismResult<Theme> {
    let mut lookup = build_lookup_with_roots(builtin_root, user_root)?;
    let path = lookup
        .remove(&name.to_lowercase())
        .ok_or_else(|| PrismError::new(format!("theme '{name}' not found")))?;
    Theme::load(&path)
}

pub fn list_available() -> PrismResult<Vec<ThemeCatalogEntry>> {
    let builtin = themes_root()?;
    let user = user_themes_dir()?;
    list_available_with_roots(&builtin, &user)
}

pub fn list_available_with_roots(
    builtin_root: &Path,
    user_root: &Path,
) -> PrismResult<Vec<ThemeCatalogEntry>> {
    let mut entries = Vec::new();
    for (path, source) in walk_theme_paths_with_roots(builtin_root, user_root)? {
        let theme = Theme::load(&path)?;
        entries.push(ThemeCatalogEntry {
            theme,
            path,
            source,
        });
    }
    let palette_dir = builtin_root.join("shared-palettes");
    if palette_dir.exists() {
        for palette in shared_palette::load_shared_palettes(&palette_dir)? {
            entries.push(ThemeCatalogEntry {
                theme: palette.to_theme(),
                path: palette.path().to_path_buf(),
                source: ThemeSource::Palette,
            });
        }
    }
    Ok(entries)
}

fn walk_theme_paths_with_roots(
    builtin_root: &Path,
    user_root: &Path,
) -> PrismResult<Vec<(PathBuf, ThemeSource)>> {
    let mut paths = Vec::new();
    if builtin_root.exists() {
        push_paths(builtin_root, ThemeSource::BuiltIn, &mut paths)?;
    }
    if user_root.exists() {
        push_paths(user_root, ThemeSource::User, &mut paths)?;
    }
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

fn build_lookup_with_roots(
    builtin_root: &Path,
    user_root: &Path,
) -> PrismResult<BTreeMap<String, PathBuf>> {
    let mut map = BTreeMap::new();
    for (path, _) in walk_theme_paths_with_roots(builtin_root, user_root)? {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PrismError::new("invalid theme filename"))?;
        map.insert(name.to_lowercase(), path);
    }
    Ok(map)
}
