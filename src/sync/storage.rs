use std::fs;
use std::path::PathBuf;

use crate::error::PrismResult;
use crate::metadata_dir;

pub fn write_metadata(key: &str, payload: &str) -> PrismResult<PathBuf> {
    let dir = metadata_dir()?;
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{key}.json"));
    fs::write(&path, payload)?;
    Ok(path)
}

pub fn read_metadata(key: &str) -> PrismResult<Option<String>> {
    let path = metadata_dir()?.join(format!("{key}.json"));
    if path.exists() {
        Ok(Some(fs::read_to_string(path)?))
    } else {
        Ok(None)
    }
}

pub fn list_backups() -> PrismResult<Vec<PathBuf>> {
    let dir = metadata_dir()?;
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut files = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }
    Ok(files)
}
