use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::PrismResult;
use crate::metadata_dir;

const CONFIG_FILE: &str = "sync-config.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncConfig {
    pub endpoint: Option<String>,
    pub ca_bundle: Option<String>,
    #[serde(default)]
    pub danger_accept_invalid_certs: bool,
}

pub fn load_config() -> PrismResult<SyncConfig> {
    let path = config_path()?;
    if path.exists() {
        let raw = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(SyncConfig::default())
    }
}

pub fn save_config(config: &SyncConfig) -> PrismResult<PathBuf> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(config)?)?;
    Ok(path)
}

pub fn config_path() -> PrismResult<PathBuf> {
    Ok(metadata_dir()?.join(CONFIG_FILE))
}
