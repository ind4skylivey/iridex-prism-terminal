use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::apply::{self, Shell};
use crate::core::color::ColorPalette;
use crate::core::prompt::PromptStyle;
use crate::ensure_config_dir;
use crate::error::{PrismError, PrismResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    #[serde(default)]
    pub colors: ColorPalette,
    #[serde(default)]
    pub prompt: PromptConfig,
    #[serde(default)]
    pub widgets: WidgetConfig,
    #[serde(default)]
    pub context_rules: ContextRules,
}

impl Theme {
    pub fn load(path: &Path) -> PrismResult<Self> {
        let raw = fs::read_to_string(path)?;
        let mut theme: Theme = toml::from_str(&raw)?;
        theme.normalize();
        theme.validate()?;
        Ok(theme)
    }

    pub fn apply(&self, shell: Shell) -> PrismResult<()> {
        let config_dir = ensure_config_dir()?;
        let script = apply::apply_theme(self, shell, &config_dir)?;
        log::info!(
            "applied theme '{}' at {}",
            self.metadata.name,
            script.display()
        );
        Ok(())
    }

    pub fn validate(&self) -> PrismResult<()> {
        self.metadata.validate()?;
        self.colors.validate()?;
        self.prompt.validate()?;
        Ok(())
    }

    fn normalize(&mut self) {
        self.prompt.ensure_segment_order();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub author: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub description: String,
}

impl ThemeMetadata {
    fn validate(&self) -> PrismResult<()> {
        if self.name.trim().is_empty() {
            return Err(PrismError::new("theme metadata requires a name"));
        }
        if self.author.trim().is_empty() {
            return Err(PrismError::new("theme metadata requires an author"));
        }
        Ok(())
    }
}

fn default_version() -> String {
    "1.0.0".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    #[serde(default)]
    pub style: PromptStyle,
    #[serde(default = "default_true")]
    pub show_user: bool,
    #[serde(default)]
    pub show_host: bool,
    #[serde(default = "default_true")]
    pub show_time: bool,
    #[serde(default = "default_true")]
    pub show_git: bool,
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default)]
    pub segments: BTreeMap<String, PromptSegment>,
    #[serde(default)]
    pub segment_order: Vec<String>,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            style: PromptStyle::Powerline,
            show_user: true,
            show_host: false,
            show_time: true,
            show_git: true,
            separator: default_separator(),
            segments: BTreeMap::new(),
            segment_order: Vec::new(),
        }
    }
}

impl PromptConfig {
    fn validate(&self) -> PrismResult<()> {
        if self.separator.trim().is_empty() {
            return Err(PrismError::new("prompt separator cannot be empty"));
        }
        Ok(())
    }

    pub fn ensure_segment_order(&mut self) {
        if self.segment_order.is_empty() {
            self.segment_order = self.segments.keys().cloned().collect();
        }
        self.segment_order
            .retain(|name| self.segments.contains_key(name));
        for key in self.segments.keys() {
            if !self.segment_order.contains(key) {
                self.segment_order.push(key.clone());
            }
        }
    }

    pub fn ordered_segments(&self) -> Vec<(String, PromptSegment)> {
        let mut ordered = Vec::new();
        for name in &self.segment_order {
            if let Some(segment) = self.segments.get(name) {
                ordered.push((name.clone(), segment.clone()));
            }
        }
        ordered
    }

    pub fn move_segment(&mut self, name: &str, delta: isize) -> bool {
        let mut moved = false;
        if let Some(idx) = self.segment_order.iter().position(|value| value == name) {
            let len = self.segment_order.len() as isize;
            let mut new_idx = idx as isize + delta;
            if new_idx < 0 {
                new_idx = 0;
            } else if new_idx >= len {
                new_idx = len - 1;
            }
            if new_idx != idx as isize {
                let value = self.segment_order.remove(idx);
                self.segment_order.insert(new_idx as usize, value);
                moved = true;
            }
        }
        moved
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromptSegment {
    #[serde(default)]
    pub bg: String,
    #[serde(default)]
    pub fg: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetConfig {
    #[serde(default)]
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextRules {
    #[serde(default)]
    pub on_git_conflict: Option<String>,
    #[serde(default)]
    pub on_high_load: Option<String>,
    #[serde(default)]
    pub night_theme: Option<String>,
    #[serde(default)]
    pub on_docker_activity: Option<String>,
    #[serde(default)]
    pub project_themes: BTreeMap<String, String>,
    #[serde(default)]
    pub priority: Vec<String>,
}

fn default_true() -> bool {
    true
}

fn default_separator() -> String {
    "".into()
}
