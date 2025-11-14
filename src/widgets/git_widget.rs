use std::time::Duration;

use colored::Colorize;

use crate::context::git::GitContext;
use crate::error::PrismResult;

use super::widget::Widget;

pub struct GitStatusWidget {
    context: GitContext,
    frame: usize,
}

impl GitStatusWidget {
    pub fn new(context: GitContext) -> Self {
        Self { context, frame: 0 }
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

    async fn render(&mut self) -> PrismResult<String> {
        let frames = ["", ""];
        let frame = frames[self.frame % frames.len()];
        self.frame = (self.frame + 1) % frames.len();
        let branch = self
            .context
            .branch
            .clone()
            .unwrap_or_else(|| "detached".into());
        let summary = format!(
            "{} {} {}",
            frame,
            branch.bold(),
            if self.context.dirty {
                "±".red()
            } else {
                "✓".green()
            }
        );
        Ok(summary)
    }
}
