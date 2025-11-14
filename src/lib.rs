pub mod cli;
pub mod context;
pub mod core;
pub mod daemon;
pub mod error;
pub mod sync;
pub mod tui;
pub mod widgets;

use std::path::{Path, PathBuf};

use dirs::config_dir;
use error::{PrismError, PrismResult};

pub const APP_NAME: &str = "prism";
pub const BRAND_NAME: &str = "IRIDEX - Adaptive Terminal Iris System";

pub fn ensure_config_dir() -> PrismResult<PathBuf> {
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
    let candidate = cwd.join("themes");
    if candidate.exists() {
        Ok(candidate)
    } else {
        Ok(cwd)
    }
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
