use std::path::{Path, PathBuf};

use crate::ensure_config_dir;
use crate::error::{PrismError, PrismResult};

pub fn tracked_files() -> PrismResult<Vec<PathBuf>> {
    let base = ensure_config_dir()?.join("dotfiles");
    if !base.exists() {
        return Ok(vec![]);
    }
    let mut files = Vec::new();
    for entry in std::fs::read_dir(base)? {
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
    std::fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join(
        path.file_name()
            .ok_or_else(|| PrismError::new("dotfile missing filename"))?,
    );
    std::fs::copy(path, &dest)?;
    Ok(dest)
}
