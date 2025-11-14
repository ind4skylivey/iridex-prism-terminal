use std::time::Duration;

use async_trait::async_trait;

use crate::error::PrismResult;

#[async_trait]
pub trait Widget: Send + Sync {
    fn name(&self) -> &str;
    fn refresh_interval(&self) -> Duration {
        Duration::from_millis(750)
    }
    fn is_enabled(&self) -> bool {
        true
    }
    async fn render(&mut self) -> PrismResult<String>;
}
