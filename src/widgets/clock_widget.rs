use std::time::Duration;

use chrono::Local;
use colored::Colorize;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::{Widget, WidgetAnimation, WidgetOutput};

pub struct ClockWidget;

impl ClockWidget {
    pub fn new() -> Self {
        Self
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

    async fn render(&mut self, _snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let frames = vec![
            format!("● {formatted}").bright_black().to_string(),
            format!("○ {formatted}").bright_black().to_string(),
        ];
        let animation = WidgetAnimation::looping(frames.clone(), Duration::from_millis(900));
        Ok(WidgetOutput::with_animation(frames[0].clone(), animation))
    }
}
