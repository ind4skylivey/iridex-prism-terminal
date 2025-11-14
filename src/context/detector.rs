use std::path::Path;

use crate::context::git::{self, GitContext};
use crate::context::project::{self, ProjectContext};
use crate::context::rules::RuleEngine;
use crate::context::system::{self, SystemContext};
use crate::context::time::{self, TimeContext};
use crate::core::theme::ContextRules;
use crate::error::PrismResult;

#[derive(Debug, Clone, Default)]
pub struct ContextSnapshot {
    pub git: GitContext,
    pub project: ProjectContext,
    pub time: TimeContext,
    pub system: SystemContext,
    pub suggested_theme: Option<String>,
}

pub struct ContextDetector {
    rules: RuleEngine,
}

impl ContextDetector {
    pub fn new(rules: Option<ContextRules>) -> Self {
        Self {
            rules: RuleEngine::new(rules.unwrap_or_default()),
        }
    }

    pub fn detect(&self, path: &Path) -> PrismResult<ContextSnapshot> {
        let git = git::detect_git_context(path).unwrap_or_default();
        let project = project::detect_project_context(path);
        let time_ctx = time::detect_time_context();
        let system_ctx = system::detect_system_context().unwrap_or_default();

        let snapshot = ContextSnapshot {
            git: git.clone(),
            project: project.clone(),
            time: time_ctx.clone(),
            system: system_ctx.clone(),
            suggested_theme: self.rules.evaluate(&git, &project, &time_ctx, &system_ctx),
        };

        Ok(snapshot)
    }
}

impl ContextSnapshot {
    pub fn summary(&self) -> String {
        format!(
            "git: {} | project: {} | time: {} | load: {:.2}%",
            self.git.branch.as_deref().unwrap_or("-"),
            self.project.language.as_deref().unwrap_or("unknown"),
            self.time.period,
            self.system.load_percent
        )
    }
}
