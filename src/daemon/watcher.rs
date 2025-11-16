use std::path::PathBuf;
use std::time::Duration;

use crate::context::{ContextDetector, ContextSnapshot};
use crate::core::loader;
use crate::core::theme::Theme;
use crate::error::PrismResult;
use crate::prompt_stream;
use crate::widgets::WidgetManager;

pub struct ContextWatcher {
    detector: ContextDetector,
    last_snapshot: Option<ContextSnapshot>,
    root: PathBuf,
    widget_manager: WidgetManager,
    current_theme: Option<Theme>,
    current_theme_name: Option<String>,
    fallback_theme: Theme,
    last_prompt: Option<String>,
    next_interval: Duration,
}

impl ContextWatcher {
    pub fn new(
        detector: ContextDetector,
        root: PathBuf,
        widget_manager: WidgetManager,
        fallback_theme: Theme,
    ) -> Self {
        Self {
            detector,
            last_snapshot: None,
            root,
            widget_manager,
            current_theme: None,
            current_theme_name: None,
            fallback_theme,
            last_prompt: None,
            next_interval: Duration::from_secs(5),
        }
    }

    pub async fn run(&mut self) -> PrismResult<()> {
        loop {
            tokio::time::sleep(self.next_interval).await;
            let snapshot = self.detector.detect(&self.root)?;
            if let Some(theme_name) = snapshot.suggested_theme.clone() {
                self.ensure_theme(&theme_name)?;
            }
            let theme = self.current_theme.as_ref().unwrap_or(&self.fallback_theme);
            let widgets = self.widget_manager.render_all(&snapshot).await?;
            let line = prompt_stream::write_prompt(theme, &snapshot, &widgets)?;
            if self
                .last_prompt
                .as_ref()
                .map(|prev| prev == &line)
                .unwrap_or(false)
            {
                // unchanged
            } else {
                self.last_prompt = Some(line);
            }
            self.last_snapshot = Some(snapshot);
            self.next_interval = self.dynamic_interval();
        }
    }

    fn dynamic_interval(&self) -> Duration {
        if let Some(snapshot) = &self.last_snapshot {
            if snapshot.system.load_percent >= 80.0 {
                return Duration::from_secs(10);
            }
            if snapshot.docker.running > 0 {
                return Duration::from_secs(3);
            }
        }
        Duration::from_secs(5)
    }

    fn ensure_theme(&mut self, theme_name: &str) -> PrismResult<()> {
        if self
            .current_theme_name
            .as_ref()
            .map(|current| current == theme_name)
            .unwrap_or(false)
        {
            return Ok(());
        }
        match loader::load_theme_by_name(theme_name) {
            Ok(theme) => {
                apply_theme(&theme)?;
                self.current_theme = Some(theme.clone());
                self.current_theme_name = Some(theme_name.to_string());
            }
            Err(err) => {
                log::warn!("Failed to load theme '{theme_name}': {err}");
            }
        }
        Ok(())
    }
}

fn apply_theme(theme: &Theme) -> PrismResult<()> {
    theme.apply(crate::core::apply::Shell::Zsh)?;
    Ok(())
}
