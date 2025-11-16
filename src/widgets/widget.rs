use std::time::Duration;

use async_trait::async_trait;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

#[derive(Debug, Clone)]
pub struct WidgetAnimation {
    pub frames: Vec<String>,
    pub interval: Duration,
    pub looping: bool,
}

impl WidgetAnimation {
    pub fn looping(frames: Vec<String>, interval: Duration) -> Self {
        Self {
            frames,
            interval,
            looping: true,
        }
    }

    pub fn one_shot(frames: Vec<String>, interval: Duration) -> Self {
        Self {
            frames,
            interval,
            looping: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WidgetOutput {
    pub content: String,
    pub animation: Option<WidgetAnimation>,
}

impl WidgetOutput {
    pub fn static_text(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            animation: None,
        }
    }

    pub fn with_animation(content: impl Into<String>, animation: WidgetAnimation) -> Self {
        Self {
            content: content.into(),
            animation: Some(animation),
        }
    }
}

impl From<String> for WidgetOutput {
    fn from(value: String) -> Self {
        WidgetOutput::static_text(value)
    }
}

impl From<&str> for WidgetOutput {
    fn from(value: &str) -> Self {
        WidgetOutput::static_text(value)
    }
}

#[async_trait]
pub trait Widget: Send + Sync {
    fn name(&self) -> &str;
    fn refresh_interval(&self) -> Duration {
        Duration::from_millis(750)
    }
    fn is_enabled(&self) -> bool {
        true
    }
    async fn render(&mut self, snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput>;
}
