use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::context::docker::DockerContext;
use crate::context::git::GitContext;
use crate::context::project::ProjectContext;
use crate::context::system::SystemContext;
use crate::context::time::TimeContext;
use crate::core::theme::ContextRules;
use crate::ensure_config_dir;
use crate::error::PrismResult;

const RULES_FILE: &str = "rules.toml";
const OVERRIDE_FILE: &str = "manual_override";

#[derive(Debug, Clone)]
pub struct RuleEngine {
    rules: ContextRules,
    override_theme: Option<String>,
    priority: Vec<RuleKey>,
}

impl RuleEngine {
    pub fn new(rules: ContextRules, override_theme: Option<String>) -> Self {
        let priority = build_priority(rules.priority.clone());
        Self {
            rules,
            override_theme,
            priority,
        }
    }

    pub fn refresh_override(&mut self, new_value: Option<String>) {
        self.override_theme = new_value;
    }

    pub fn evaluate(
        &self,
        git: &GitContext,
        project: &ProjectContext,
        time: &TimeContext,
        system: &SystemContext,
        docker: &DockerContext,
    ) -> Option<String> {
        for key in &self.priority {
            if let Some(theme) = self.eval_rule(*key, git, project, time, system, docker) {
                return Some(theme);
            }
        }
        None
    }

    fn eval_rule(
        &self,
        key: RuleKey,
        git: &GitContext,
        project: &ProjectContext,
        time: &TimeContext,
        system: &SystemContext,
        docker: &DockerContext,
    ) -> Option<String> {
        match key {
            RuleKey::ManualOverride => self.override_theme.clone(),
            RuleKey::GitConflict => self
                .rules
                .on_git_conflict
                .clone()
                .filter(|_| git.has_conflict),
            RuleKey::HighLoad => self
                .rules
                .on_high_load
                .clone()
                .filter(|_| system.load_percent >= 80.0),
            RuleKey::NightTime => self
                .rules
                .night_theme
                .clone()
                .filter(|_| time.period.contains("evening") || time.period.contains("night")),
            RuleKey::ProjectLanguage => project
                .language
                .as_ref()
                .and_then(|lang| self.rules.project_themes.get(lang))
                .cloned(),
            RuleKey::DockerActive => self
                .rules
                .on_docker_activity
                .clone()
                .filter(|_| docker.running > 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RuleKey {
    ManualOverride,
    GitConflict,
    HighLoad,
    NightTime,
    ProjectLanguage,
    DockerActive,
}

fn build_priority(order: Vec<String>) -> Vec<RuleKey> {
    if order.is_empty() {
        return vec![
            RuleKey::ManualOverride,
            RuleKey::GitConflict,
            RuleKey::HighLoad,
            RuleKey::NightTime,
            RuleKey::ProjectLanguage,
            RuleKey::DockerActive,
        ];
    }
    let mut dedup = Vec::new();
    let mut seen = HashSet::new();
    for value in order {
        if let Some(key) = key_from_str(&value) {
            if seen.insert(key as u8) {
                dedup.push(key);
            }
        }
    }
    if dedup.is_empty() {
        build_priority(vec![])
    } else {
        dedup
    }
}

fn key_from_str(value: &str) -> Option<RuleKey> {
    match value {
        "manual_override" => Some(RuleKey::ManualOverride),
        "git_conflict" => Some(RuleKey::GitConflict),
        "high_load" => Some(RuleKey::HighLoad),
        "night" => Some(RuleKey::NightTime),
        "project" => Some(RuleKey::ProjectLanguage),
        "docker" => Some(RuleKey::DockerActive),
        _ => None,
    }
}

pub fn rules_path() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join(RULES_FILE))
}

pub fn override_path() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join(OVERRIDE_FILE))
}

pub fn load_rules() -> PrismResult<ContextRules> {
    let path = rules_path()?;
    if !path.exists() {
        let template = ContextRules::default();
        write_rules_template(&path, &template)?;
        return Ok(template);
    }
    let raw = fs::read_to_string(path)?;
    Ok(toml::from_str(&raw)?)
}

fn write_rules_template(path: &Path, rules: &ContextRules) -> PrismResult<()> {
    let mut defaults = toml::to_string_pretty(rules)?;
    defaults.push_str("\n# project_themes examples\n# [project_themes]\n# rust = \"cyberpunk\"\n");
    fs::write(path, defaults)?;
    Ok(())
}

pub fn save_rules(rules: &ContextRules) -> PrismResult<()> {
    let path = rules_path()?;
    let payload = toml::to_string_pretty(rules)?;
    fs::write(path, payload)?;
    Ok(())
}

pub fn load_manual_override() -> PrismResult<Option<String>> {
    let path = override_path()?;
    if path.exists() {
        Ok(Some(fs::read_to_string(path)?.trim().to_string()))
    } else {
        Ok(None)
    }
}

pub fn write_manual_override(value: Option<&str>) -> PrismResult<()> {
    let path = override_path()?;
    if let Some(theme) = value {
        fs::write(path, theme)?;
    } else if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
