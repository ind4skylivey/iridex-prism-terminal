use serde::{Deserialize, Serialize};

use crate::error::PrismResult;
use crate::metadata_dir;

const STATE_FILE: &str = "sync-state.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncState {
    pub last_pull: Option<String>,
    pub last_push: Option<String>,
    pub last_remote: Option<String>,
}

pub fn load_state() -> PrismResult<SyncState> {
    let path = metadata_dir()?.join(STATE_FILE);
    if path.exists() {
        let raw = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(SyncState::default())
    }
}

pub fn save_state(state: &SyncState) -> PrismResult<()> {
    let path = metadata_dir()?.join(STATE_FILE);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(state)?)?;
    Ok(())
}
