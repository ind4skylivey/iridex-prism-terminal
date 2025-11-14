use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ensure_config_dir;
use crate::error::PrismResult;

const STATE_FILE: &str = "sync-state.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncState {
    pub last_push: Option<String>,
    pub last_pull: Option<String>,
    pub last_remote: Option<String>,
}

pub fn load_state() -> PrismResult<SyncState> {
    let path = state_path()?;
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(SyncState::default())
    }
}

pub fn save_state(state: &SyncState) -> PrismResult<()> {
    let path = state_path()?;
    fs::write(path, serde_json::to_string_pretty(state)?)?;
    Ok(())
}

fn state_path() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join(STATE_FILE))
}
