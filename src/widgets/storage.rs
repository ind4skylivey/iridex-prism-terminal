use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::PrismResult;
use crate::widgets::preferences::WidgetPreferences;

const ENABLED_FILE: &str = "widgets.json";
const CONFIG_FILE: &str = "widgets-config.json";

pub fn load_enabled(dir: &Path) -> PrismResult<Vec<String>> {
    let path = enabled_path(dir);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(vec![])
    }
}

pub fn save_enabled(dir: &Path, widgets: &[String]) -> PrismResult<()> {
    let path = enabled_path(dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(widgets)?)?;
    Ok(())
}

pub fn load_settings(dir: &Path) -> PrismResult<BTreeMap<String, BTreeMap<String, String>>> {
    let path = config_path(dir);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(BTreeMap::new())
    }
}

pub fn save_settings(
    dir: &Path,
    config: &BTreeMap<String, BTreeMap<String, String>>,
) -> PrismResult<()> {
    let path = config_path(dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(config)?)?;
    Ok(())
}

pub fn load_preferences(dir: &Path) -> PrismResult<BTreeMap<String, WidgetPreferences>> {
    let raw = load_settings(dir)?;
    let mut result = BTreeMap::new();
    for (name, values) in raw {
        let prefs = WidgetPreferences::from_map(&values);
        result.insert(name, prefs);
    }
    Ok(result)
}

pub fn enabled_path(dir: &Path) -> PathBuf {
    dir.join(ENABLED_FILE)
}

pub fn config_path(dir: &Path) -> PathBuf {
    dir.join(CONFIG_FILE)
}
