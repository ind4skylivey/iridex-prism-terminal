use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::ensure_config_dir;
use crate::error::PrismResult;
use crate::sync::client::SyncData;

const HISTORY_DIR: &str = "history";
const INDEX_FILE: &str = "index.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub action: String,
    pub timestamp: String,
    pub snapshot_path: String,
}

pub fn record_snapshot(action: &str, snapshot: &SyncData) -> PrismResult<()> {
    let dir = history_dir()?;
    fs::create_dir_all(&dir)?;
    let filename = format!("{}-{action}.json", sanitize(&snapshot.timestamp));
    let path = dir.join(filename);
    fs::write(&path, serde_json::to_string_pretty(snapshot)?)?;

    let mut entries = read_entries(&dir)?;
    entries.push(HistoryEntry {
        action: action.into(),
        timestamp: snapshot.timestamp.clone(),
        snapshot_path: path.to_string_lossy().to_string(),
    });
    write_entries(&dir, &entries)?;
    Ok(())
}

pub fn list() -> PrismResult<Vec<HistoryEntry>> {
    read_entries(&history_dir()?)
}

pub fn latest() -> PrismResult<Option<HistoryEntry>> {
    Ok(list()?.into_iter().last())
}

pub fn load_snapshot(path: &str) -> PrismResult<SyncData> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn history_dir() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join(HISTORY_DIR))
}

fn read_entries(dir: &Path) -> PrismResult<Vec<HistoryEntry>> {
    let path = dir.join(INDEX_FILE);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(Vec::new())
    }
}

fn write_entries(dir: &Path, entries: &[HistoryEntry]) -> PrismResult<()> {
    let path = dir.join(INDEX_FILE);
    fs::write(path, serde_json::to_string_pretty(entries)?)?;
    Ok(())
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}
