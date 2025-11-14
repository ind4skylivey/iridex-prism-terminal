use std::time::{Duration, SystemTime};

use chrono::Local;
use colored::Colorize;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::Widget;

pub struct ClockWidget {
    heartbeat: SystemTime,
}

impl ClockWidget {
    pub fn new() -> Self {
        Self {
            heartbeat: SystemTime::now(),
        }
    }
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Widget for ClockWidget {
    fn name(&self) -> &str {
        "clock"
    }

    fn refresh_interval(&self) -> Duration {
        Duration::from_millis(1000)
    }

    async fn render(&mut self, _snapshot: &ContextSnapshot) -> PrismResult<String> {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let pulse = if self
            .heartbeat
            .elapsed()
            .map(|d| d.as_secs() % 2 == 0)
            .unwrap_or(true)
        {
            "●"
        } else {
            "○"
        };
        Ok(format!("{pulse} {formatted}").bright_black().to_string())
    }
}
