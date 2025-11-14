use std::fs;

use crate::context::git::GitContext;
use crate::context::project::ProjectContext;
use crate::context::system::SystemContext;
use crate::context::time::TimeContext;
use crate::core::theme::ContextRules;
use crate::ensure_config_dir;
use crate::error::PrismResult;

const RULES_FILE: &str = "context-rules.toml";
const OVERRIDE_FILE: &str = "context-override.toml";

#[derive(Debug, Clone)]
pub struct RuleEngine {
    rules: ContextRules,
}

impl RuleEngine {
    pub fn new(rules: ContextRules) -> Self {
        Self { rules }
    }

    pub fn evaluate(
        &self,
        git: &GitContext,
        project: &ProjectContext,
        time: &TimeContext,
        system: &SystemContext,
    ) -> Option<String> {
        if git.has_conflict {
            if let Some(theme) = &self.rules.on_git_conflict {
                return Some(theme.clone());
            }
        }
        if system.load_percent >= 80.0 {
            if let Some(theme) = &self.rules.on_high_load {
                return Some(theme.clone());
            }
        }
        if time.period.contains("evening") || time.period.contains("night") {
            if let Some(theme) = &self.rules.night_theme {
                return Some(theme.clone());
            }
        }
        if let Some(language) = &project.language {
            if let Some(theme) = self.rules.project_themes.get(language) {
                return Some(theme.clone());
            }
        }
        None
    }
}

pub fn load_rules() -> PrismResult<Option<ContextRules>> {
    load_rules_from(RULES_FILE)
}

pub fn load_manual_override() -> PrismResult<Option<ContextRules>> {
    load_rules_from(OVERRIDE_FILE)
}

fn load_rules_from(file: &str) -> PrismResult<Option<ContextRules>> {
    let path = ensure_config_dir()?.join(file);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(Some(toml::from_str(&raw)?))
    } else {
        Ok(None)
    }
}
