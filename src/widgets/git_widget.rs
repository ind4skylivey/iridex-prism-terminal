use std::time::Duration;

use colored::Colorize;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::{Widget, WidgetAnimation, WidgetOutput};

pub struct GitStatusWidget;

impl GitStatusWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitStatusWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Widget for GitStatusWidget {
    fn name(&self) -> &str {
        "git-status"
    }

    fn refresh_interval(&self) -> Duration {
        Duration::from_millis(500)
    }

    async fn render(&mut self, snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
        let branch = snapshot
            .git
            .branch
            .clone()
            .unwrap_or_else(|| "detached".into());
        let status = if snapshot.git.has_conflict {
            "⚠".yellow().to_string()
        } else if snapshot.git.dirty {
            "±".red().to_string()
        } else {
            "✓".green().to_string()
        };
        let divergence = match (snapshot.git.ahead, snapshot.git.behind) {
            (0, 0) => String::new(),
            (ahead, behind) => format!(" ⇡{ahead} ⇣{behind}"),
        };
        let summary = format!(" {} {}{}", branch.bold(), status, divergence);
        if snapshot.git.dirty
            || snapshot.git.has_conflict
            || snapshot.git.ahead + snapshot.git.behind > 0
        {
            let spinner = ["⠋", "⠙", "⠸", "⠴", "⠦", "⠇"];
            let frames = spinner
                .into_iter()
                .map(|glyph| format!("{glyph} {summary}"))
                .collect::<Vec<_>>();
            let animation = WidgetAnimation::looping(frames.clone(), Duration::from_millis(120));
            Ok(WidgetOutput::with_animation(frames[0].clone(), animation))
        } else {
            Ok(WidgetOutput::static_text(summary))
        }
    }
}
