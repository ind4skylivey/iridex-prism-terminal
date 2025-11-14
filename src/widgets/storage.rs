use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::PrismResult;

const ENABLED_FILE: &str = "widgets.json";
const SETTINGS_FILE: &str = "widget-settings.json";

pub type WidgetSettings = BTreeMap<String, BTreeMap<String, String>>;

pub fn load_enabled(config_dir: &Path) -> PrismResult<Vec<String>> {
    let path = enabled_path(config_dir);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(Vec::new())
    }
}

pub fn save_enabled(config_dir: &Path, widgets: &[String]) -> PrismResult<()> {
    let path = enabled_path(config_dir);
    fs::write(path, serde_json::to_string_pretty(widgets)?)?;
    Ok(())
}

pub fn load_settings(config_dir: &Path) -> PrismResult<WidgetSettings> {
    let path = settings_path(config_dir);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(BTreeMap::new())
    }
}

pub fn save_settings(config_dir: &Path, settings: &WidgetSettings) -> PrismResult<()> {
    let path = settings_path(config_dir);
    fs::write(path, serde_json::to_string_pretty(settings)?)?;
    Ok(())
}

pub fn upsert_setting(config_dir: &Path, widget: &str, key: &str, value: &str) -> PrismResult<()> {
    let mut settings = load_settings(config_dir)?;
    let entry = settings.entry(widget.to_string()).or_default();
    entry.insert(key.to_string(), value.to_string());
    save_settings(config_dir, &settings)
}

pub fn widget_settings_path(config_dir: &Path) -> PathBuf {
    settings_path(config_dir)
}

fn enabled_path(config_dir: &Path) -> PathBuf {
    config_dir.join(ENABLED_FILE)
}

fn settings_path(config_dir: &Path) -> PathBuf {
    config_dir.join(SETTINGS_FILE)
}
