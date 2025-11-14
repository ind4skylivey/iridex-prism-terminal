use std::time::Duration;

use colored::Colorize;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::Widget;

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

    async fn render(&mut self, snapshot: &ContextSnapshot) -> PrismResult<String> {
        let running = snapshot.docker.running;
        let total = snapshot.docker.total;
        Ok(format!("Docker {running}/{total}").yellow().to_string())
    }
}
