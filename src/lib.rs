pub mod assets;
pub mod catalog;
pub mod cli;
pub mod context;
pub mod core;
pub mod daemon;
pub mod error;
pub mod prompt_stream;
pub mod sync;
pub mod themes;
pub mod tui;
pub mod widgets;

use std::path::{Path, PathBuf};

use dirs::config_dir;
use error::{PrismError, PrismResult};

pub const APP_NAME: &str = "prism";
pub const BRAND_NAME: &str = "IRIDEX - Adaptive Terminal Iris System";

pub fn ensure_config_dir() -> PrismResult<PathBuf> {
    if let Ok(custom) = std::env::var("PRISM_CONFIG_DIR") {
        let path = PathBuf::from(custom);
        std::fs::create_dir_all(&path)?;
        return Ok(path);
    }
    let dir = config_dir()
        .ok_or_else(|| PrismError::new("could not resolve config directory"))?
        .join(APP_NAME);
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn themes_root() -> PrismResult<PathBuf> {
    if let Ok(dir) = std::env::var("PRISM_THEME_DIR") {
        return Ok(PathBuf::from(dir));
    }

    let cwd = std::env::current_dir()?;
    let local_candidate = cwd.join("themes");
    // If we are in the repo (dev mode), use local files
    if local_candidate.exists() && local_candidate.join("shared-palettes").exists() {
        return Ok(local_candidate);
    }

    // Otherwise, use the managed directory in ~/.config/prism/themes
    let managed_dir = ensure_config_dir()?.join("themes");
    
    // Check if we need to extract (if empty or missing)
    // We check for a marker file or just "shared-palettes"
    if !managed_dir.join("shared-palettes").exists() {
        extract_embedded_themes(&managed_dir)?;
    }

    Ok(managed_dir)
}

fn extract_embedded_themes(target: &Path) -> PrismResult<()> {
    use crate::assets::ThemeAssets;
    
    // println!("Extracting built-in themes to {}...", target.display());
    
    for file in ThemeAssets::iter() {
        let content = ThemeAssets::get(file.as_ref()).unwrap();
        let path = target.join(file.as_ref());
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, content.data)?;
    }
    
    Ok(())
}

pub fn data_dir() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join("data"))
}

pub fn user_themes_dir() -> PrismResult<PathBuf> {
    let dir = ensure_config_dir()?.join("themes");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn metadata_dir() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join("metadata"))
}

pub fn repo_relative(path: impl AsRef<Path>) -> PrismResult<PathBuf> {
    let root = std::env::current_dir()?;
    Ok(root.join(path))
}
