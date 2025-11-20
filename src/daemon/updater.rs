use std::time::Duration;

use tokio::time::interval;

use crate::error::PrismResult;

pub struct WidgetUpdater;

impl WidgetUpdater {
    pub async fn run(&self) -> PrismResult<()> {
        let mut ticker = interval(Duration::from_secs(2));
        loop {
            ticker.tick().await;
            log::trace!("widget updater heartbeat");
        }
    }
}
