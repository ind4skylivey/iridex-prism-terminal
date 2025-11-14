use std::path::PathBuf;
use std::time::Duration;

use tokio::time::interval;

use crate::context::{ContextDetector, ContextSnapshot};
use crate::core::loader;
use crate::core::theme::Theme;
use crate::error::PrismResult;

pub struct ContextWatcher {
    detector: ContextDetector,
    last_snapshot: Option<ContextSnapshot>,
    root: PathBuf,
}

impl ContextWatcher {
    pub fn new(detector: ContextDetector, root: PathBuf) -> Self {
        Self {
            detector,
            last_snapshot: None,
            root,
        }
    }

    pub async fn run(&mut self) -> PrismResult<()> {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            let snapshot = self.detector.detect(&self.root)?;
            if self
                .last_snapshot
                .as_ref()
                .map(|prev| prev.suggested_theme != snapshot.suggested_theme)
                .unwrap_or(true)
            {
                if let Some(theme_name) = snapshot.suggested_theme.clone() {
                    if let Ok(theme) = loader::load_theme_by_name(&theme_name) {
                        apply_theme(&theme)?;
                    }
                }
                self.last_snapshot = Some(snapshot);
            }
        }
    }
}

fn apply_theme(theme: &Theme) -> PrismResult<()> {
    theme.apply(crate::core::apply::Shell::Zsh)?;
    Ok(())
}
