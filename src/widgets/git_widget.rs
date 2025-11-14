use std::time::Duration;

use colored::Colorize;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::Widget;

pub struct GitStatusWidget {
    frame: usize,
}

impl GitStatusWidget {
    pub fn new() -> Self {
        Self { frame: 0 }
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

    async fn render(&mut self, snapshot: &ContextSnapshot) -> PrismResult<String> {
        let frames = ["", ""];
        let frame = frames[self.frame % frames.len()];
        self.frame = (self.frame + 1) % frames.len();
        let branch = snapshot
            .git
            .branch
            .clone()
            .unwrap_or_else(|| "detached".into());
        let summary = format!(
            "{} {} {}",
            frame,
            branch.bold(),
            if snapshot.git.dirty {
                "±".red()
            } else {
                "✓".green()
            }
        );
        Ok(summary)
    }
}
