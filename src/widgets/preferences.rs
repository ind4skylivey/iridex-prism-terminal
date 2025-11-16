use std::collections::BTreeMap;
use std::time::Duration;

use log::warn;

#[derive(Clone, Debug, Default)]
pub struct WidgetPreferences {
    pub enabled: Option<bool>,
    pub refresh_interval: Option<Duration>,
}

impl WidgetPreferences {
    pub fn from_map(values: &BTreeMap<String, String>) -> Self {
        let mut prefs = Self::default();
        for (key, value) in values {
            let normalized = key.trim().to_ascii_lowercase();
            match normalized.as_str() {
                "enabled" => match value.trim().parse::<bool>() {
                    Ok(parsed) => prefs.enabled = Some(parsed),
                    Err(_) => warn!("Invalid enabled value for widget '{}': {}", key, value),
                },
                "refresh_interval" | "refresh_interval_ms" | "interval_ms" => {
                    if let Some(duration) = parse_duration(value) {
                        prefs.refresh_interval = Some(duration);
                    } else {
                        warn!("Invalid refresh interval for widget '{}': {}", key, value);
                    }
                }
                _ => continue,
            }
        }
        prefs
    }
}

fn parse_duration(value: &str) -> Option<Duration> {
    let trimmed = value.trim();
    if let Ok(ms) = trimmed.parse::<u64>() {
        return Some(Duration::from_millis(ms));
    }
    if let Some(stripped) = trimmed.strip_suffix("ms") {
        return stripped
            .trim()
            .parse::<u64>()
            .ok()
            .map(Duration::from_millis);
    }
    if let Some(stripped) = trimmed.strip_suffix('s') {
        return stripped.trim().parse::<u64>().ok().map(Duration::from_secs);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn parses_basic_preferences() {
        let mut raw = BTreeMap::new();
        raw.insert("enabled".to_string(), "false".to_string());
        raw.insert("refresh_interval_ms".to_string(), "500".to_string());
        let prefs = WidgetPreferences::from_map(&raw);
        assert_eq!(prefs.enabled, Some(false));
        assert_eq!(prefs.refresh_interval, Some(Duration::from_millis(500)));
    }

    #[test]
    fn allows_seconds_suffix() {
        let mut raw = BTreeMap::new();
        raw.insert("refresh_interval".to_string(), "2s".to_string());
        let prefs = WidgetPreferences::from_map(&raw);
        assert_eq!(prefs.refresh_interval, Some(Duration::from_secs(2)));
    }

    #[test]
    fn ignores_unknown_values() {
        let mut raw = BTreeMap::new();
        raw.insert("refresh_interval".to_string(), "nonsense".to_string());
        raw.insert("enabled".to_string(), "maybe".to_string());
        let prefs = WidgetPreferences::from_map(&raw);
        assert_eq!(prefs.enabled, None);
        assert_eq!(prefs.refresh_interval, None);
    }
}
