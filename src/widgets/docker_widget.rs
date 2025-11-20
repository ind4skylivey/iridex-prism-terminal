use std::time::Duration;

use colored::Colorize;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::{Widget, WidgetAnimation, WidgetOutput};

pub struct DockerWidget;

impl DockerWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DockerWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Widget for DockerWidget {
    fn name(&self) -> &str {
        "docker"
    }

    fn refresh_interval(&self) -> Duration {
        Duration::from_secs(2)
    }

    async fn render(&mut self, snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
        let running = snapshot.docker.running;
        let total = snapshot.docker.total;
        let summary = format!("Docker {running}/{total}").yellow().to_string();
        if running > 0 {
            let frames = ["🐳", "🐋", "🐬", "🐟"]
                .into_iter()
                .map(|icon| format!("{icon} {summary}"))
                .collect::<Vec<_>>();
            let animation = WidgetAnimation::looping(frames.clone(), Duration::from_millis(700));
            Ok(WidgetOutput::with_animation(frames[0].clone(), animation))
        } else {
            Ok(WidgetOutput::static_text(summary))
        }
    }
}
