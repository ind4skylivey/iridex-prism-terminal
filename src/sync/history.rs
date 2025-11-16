use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::PrismResult;
use crate::metadata_dir;
use crate::sync::storage;
use crate::sync::SyncData;

const HISTORY_FILE: &str = "sync-history.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub action: String,
    pub timestamp: String,
    pub snapshot_path: String,
}

pub fn record(action: &str, snapshot_path: &Path) -> PrismResult<()> {
    let mut entries = load_all()?;
    entries.push(HistoryEntry {
        action: action.into(),
        timestamp: chrono::Local::now().to_rfc3339(),
        snapshot_path: snapshot_path.display().to_string(),
    });
    save_all(&entries)
}

pub fn list() -> PrismResult<Vec<HistoryEntry>> {
    let mut entries = load_all()?;
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(entries)
}

pub fn latest() -> PrismResult<Option<HistoryEntry>> {
    let mut entries = load_all()?;
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(entries.pop())
}

fn load_all() -> PrismResult<Vec<HistoryEntry>> {
    let path = history_path()?;
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(vec![])
    }
}

fn save_all(entries: &[HistoryEntry]) -> PrismResult<()> {
    let path = history_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(entries)?)?;
    Ok(())
}

fn history_path() -> PrismResult<PathBuf> {
    Ok(metadata_dir()?.join(HISTORY_FILE))
}

pub fn record_snapshot(action: &str, payload: &SyncData) -> PrismResult<()> {
    let serialized = serde_json::to_string_pretty(payload)?;
    let key = format!(
        "sync-{}-{}",
        chrono::Local::now().format("%Y%m%d%H%M%S"),
        action
    );
    let path = storage::write_metadata(&key, &serialized)?;
    record(action, &path)
}

pub fn load_snapshot(path: &str) -> PrismResult<SyncData> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}
