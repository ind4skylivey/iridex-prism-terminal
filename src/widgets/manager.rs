use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::clock_widget::ClockWidget;
use super::docker_widget::DockerWidget;
use super::git_widget::GitStatusWidget;
use super::system_widget::SystemWidget;
use super::widget::Widget;

pub struct WidgetManager {
    widgets: Vec<ManagedWidget>,
}

struct ManagedWidget {
    handle: Arc<Mutex<Box<dyn Widget>>>,
    last_render: Option<Instant>,
    cache: Option<String>,
    interval: Duration,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn with_widgets(mut self, widgets: Vec<Box<dyn Widget>>) -> Self {
        self.widgets = widgets.into_iter().map(ManagedWidget::new).collect();
        self
    }

    pub fn register(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(ManagedWidget::new(widget));
    }

    pub async fn render_all(&mut self, snapshot: &ContextSnapshot) -> PrismResult<Vec<String>> {
        let mut outputs = Vec::with_capacity(self.widgets.len());
        for managed in &mut self.widgets {
            let mut guard = managed.handle.lock().await;
            if !guard.is_enabled() {
                continue;
            }
            let should_render = managed
                .last_render
                .map(|instant| instant.elapsed() >= managed.interval)
                .unwrap_or(true);
            if should_render {
                let output = guard.render(snapshot).await?;
                managed.cache = Some(output);
                managed.last_render = Some(Instant::now());
            }
            if let Some(value) = &managed.cache {
                outputs.push(value.clone());
            }
        }
        Ok(outputs)
    }

    pub fn from_names(names: &[String]) -> Self {
        let mut manager = Self::new();
        for name in names {
            if let Some(widget) = build_widget(name) {
                manager.register(widget);
            } else {
                log::warn!("Unknown widget '{name}', skipping");
            }
        }
        manager
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

fn build_widget(name: &str) -> Option<Box<dyn Widget>> {
    match name {
        "clock" => Some(Box::new(ClockWidget::new())),
        "system" => Some(Box::new(SystemWidget::new())),
        "docker" => Some(Box::new(DockerWidget::new())),
        "git" => Some(Box::new(GitStatusWidget::new())),
        _ => None,
    }
}

impl ManagedWidget {
    fn new(widget: Box<dyn Widget>) -> Self {
        let interval = widget.refresh_interval();
        Self {
            handle: Arc::new(Mutex::new(widget)),
            last_render: None,
            cache: None,
            interval,
        }
    }
}
