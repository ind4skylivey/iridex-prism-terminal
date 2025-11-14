use std::time::Duration;

use colored::Colorize;

use crate::error::PrismResult;

use super::widget::Widget;

pub struct DockerWidget {
    last_count: u32,
}

impl DockerWidget {
    pub fn new() -> Self {
        Self { last_count: 0 }
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

    async fn render(&mut self) -> PrismResult<String> {
        let simulated = std::env::var("PRISM_DOCKER_COUNT")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(self.last_count);
        self.last_count = simulated;
        Ok(format!("Containers {simulated}").yellow().to_string())
    }
}
