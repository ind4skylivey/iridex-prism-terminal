use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;
use tokio::task::JoinSet;

use crate::context::ContextSnapshot;
use crate::error::{PrismError, PrismResult};

use super::clock_widget::ClockWidget;
use super::docker_widget::DockerWidget;
use super::git_widget::GitStatusWidget;
use super::plugin::PluginRegistry;
use super::system_widget::SystemWidget;
use super::widget::{Widget, WidgetAnimation, WidgetOutput};
use crate::widgets::preferences::WidgetPreferences;
use crate::widgets::storage;

pub struct WidgetManager {
    widgets: Vec<ManagedWidget>,
}

struct ManagedWidget {
    name: String,
    handle: Arc<Mutex<Box<dyn Widget>>>,
    last_render: Option<Instant>,
    cache: Option<String>,
    interval: Duration,
    animation: Option<AnimationState>,
    enabled: bool,
}

struct AnimationState {
    frames: Vec<String>,
    index: usize,
    interval: Duration,
    looping: bool,
    last_tick: Instant,
    finished: bool,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn with_widgets(mut self, widgets: Vec<Box<dyn Widget>>) -> Self {
        self.widgets = widgets
            .into_iter()
            .map(|widget| ManagedWidget::new_with(widget, None))
            .collect();
        self
    }

    pub fn register(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(ManagedWidget::new_with(widget, None));
    }

    fn register_with_config(
        &mut self,
        widget: Box<dyn Widget>,
        preferences: Option<WidgetPreferences>,
    ) {
        self.widgets
            .push(ManagedWidget::new_with(widget, preferences));
    }

    pub async fn render_all(&mut self, snapshot: &ContextSnapshot) -> PrismResult<Vec<String>> {
        let mut outputs = Vec::with_capacity(self.widgets.len());
        let shared_snapshot = Arc::new(snapshot.clone());
        let mut join_set = JoinSet::new();

        for idx in 0..self.widgets.len() {
            if !self.widgets[idx].should_render() {
                continue;
            }
            let handle = Arc::clone(&self.widgets[idx].handle);
            let snapshot = shared_snapshot.clone();
            let name = self.widgets[idx].name.clone();
            join_set.spawn(async move {
                let mut guard = handle.lock().await;
                if !guard.is_enabled() {
                    return Ok::<_, PrismError>((idx, name, None));
                }
                let start = Instant::now();
                let output = guard.render(&snapshot).await?;
                let elapsed = start.elapsed();
                Ok::<_, PrismError>((idx, name, Some((output, elapsed))))
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok((idx, name, Some((output, elapsed))))) => {
                    self.widgets[idx].apply_output(output);
                    if elapsed >= Duration::from_millis(250) {
                        log::debug!("Widget {name} render took {:?}", elapsed);
                    }
                }
                Ok(Ok((idx, name, None))) => {
                    log::debug!("Widget {name} disabled itself; clearing cache");
                    self.widgets[idx].mark_disabled();
                }
                Ok(Err(err)) => {
                    log::warn!("Widget render failed: {err}");
                }
                Err(err) => {
                    log::warn!("Widget task join error: {err}");
                }
            }
        }

        for managed in &mut self.widgets {
            managed.advance_animation();
            if managed.enabled {
                if let Some(value) = managed.cache.as_ref() {
                    outputs.push(value.clone());
                }
            }
        }
        Ok(outputs)
    }

    pub fn from_names(names: &[String], config_dir: &Path) -> PrismResult<Self> {
        let registry = PluginRegistry::load(config_dir)?;
        let mut manager = Self::new();
        let preferences = storage::load_preferences(config_dir)?;
        for name in names {
            match build_widget(name, &registry) {
                Ok(Some(widget)) => {
                    let prefs = preferences.get(name).cloned();
                    manager.register_with_config(widget, prefs);
                }
                Ok(None) => log::warn!("Unknown widget '{name}', skipping"),
                Err(err) => log::warn!("Failed to initialize widget '{name}': {err}"),
            }
        }
        Ok(manager)
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

fn build_widget(name: &str, registry: &PluginRegistry) -> PrismResult<Option<Box<dyn Widget>>> {
    let builtin: Option<Box<dyn Widget>> = match name {
        "clock" => Some(Box::new(ClockWidget::new())),
        "system" => Some(Box::new(SystemWidget::new())),
        "docker" => Some(Box::new(DockerWidget::new())),
        "git" => Some(Box::new(GitStatusWidget::new())),
        _ => None,
    };
    if builtin.is_some() {
        return Ok(builtin);
    }
    if let Some(plugin) = registry.instantiate(name) {
        return Ok(Some(Box::new(plugin)));
    }
    Ok(None)
}

impl ManagedWidget {
    fn new_with(widget: Box<dyn Widget>, preferences: Option<WidgetPreferences>) -> Self {
        let interval = if let Some(pref) = preferences.as_ref() {
            pref.refresh_interval
                .unwrap_or_else(|| widget.refresh_interval())
        } else {
            widget.refresh_interval()
        };
        let name = widget.name().to_string();
        let handle = Arc::new(Mutex::new(widget));
        let enabled = preferences
            .as_ref()
            .and_then(|pref| pref.enabled)
            .unwrap_or(true);
        Self {
            name,
            handle,
            last_render: None,
            cache: None,
            interval,
            animation: None,
            enabled,
        }
    }

    fn should_render(&self) -> bool {
        if !self.enabled && self.cache.is_none() {
            return false;
        }
        self.last_render
            .map(|instant| instant.elapsed() >= self.interval)
            .unwrap_or(true)
    }

    fn apply_output(&mut self, output: WidgetOutput) {
        self.enabled = true;
        self.last_render = Some(Instant::now());
        if let Some(animation) = output.animation {
            if let Some((state, first_frame)) = AnimationState::activate(animation) {
                self.cache = Some(first_frame);
                self.animation = Some(state);
                return;
            }
        }
        self.cache = Some(output.content);
        self.animation = None;
    }

    fn mark_disabled(&mut self) {
        self.enabled = false;
        self.cache = None;
        self.animation = None;
    }

    fn advance_animation(&mut self) {
        if let Some(state) = self.animation.as_mut() {
            if let Some(frame) = state.poll() {
                self.cache = Some(frame);
            }
            if state.is_finished() {
                self.animation = None;
            }
        }
    }
}

impl AnimationState {
    fn activate(spec: WidgetAnimation) -> Option<(Self, String)> {
        if spec.frames.is_empty() {
            return None;
        }
        let first = spec.frames[0].clone();
        let WidgetAnimation {
            frames,
            interval,
            looping,
        } = spec;
        let finished = frames.len() <= 1 && !looping;
        Some((
            Self {
                frames,
                index: 0,
                interval,
                looping,
                last_tick: Instant::now(),
                finished,
            },
            first,
        ))
    }

    fn poll(&mut self) -> Option<String> {
        if self.finished || self.frames.len() <= 1 {
            if !self.looping {
                self.finished = true;
            }
            return None;
        }
        if self.last_tick.elapsed() < self.interval {
            return None;
        }
        self.last_tick = Instant::now();
        if self.index + 1 >= self.frames.len() {
            if self.looping {
                self.index = 0;
            } else {
                self.finished = true;
                return None;
            }
        } else {
            self.index += 1;
        }
        Some(self.frames[self.index].clone())
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ContextSnapshot;
    use async_trait::async_trait;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;
    use tokio::runtime::Runtime;

    struct TestWidget {
        name: &'static str,
        interval: Duration,
    }

    impl TestWidget {
        fn new(name: &'static str, interval: Duration) -> Self {
            Self { name, interval }
        }
    }

    #[async_trait]
    impl Widget for TestWidget {
        fn name(&self) -> &str {
            self.name
        }

        fn refresh_interval(&self) -> Duration {
            self.interval
        }

        async fn render(&mut self, _snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
            Ok(WidgetOutput::static_text("ok"))
        }
    }

    #[test]
    fn managed_widget_applies_preferences() {
        let prefs = WidgetPreferences {
            enabled: Some(false),
            refresh_interval: Some(Duration::from_secs(5)),
        };
        let widget = Box::new(TestWidget::new("dummy", Duration::from_millis(250)));
        let managed = ManagedWidget::new_with(widget, Some(prefs));
        assert_eq!(managed.interval, Duration::from_secs(5));
        assert!(!managed.enabled);
        assert!(!managed.should_render());
    }

    #[test]
    fn manager_respects_disabled_preference() {
        let dir = TempDir::new().expect("config dir");
        let config_path = dir.path().join("widgets-config.json");
        let settings = json!({
            "clock": { "enabled": "false" }
        });
        fs::write(&config_path, serde_json::to_string(&settings).unwrap()).unwrap();
        let names = vec!["clock".into()];
        let mut manager = WidgetManager::from_names(&names, dir.path()).unwrap();
        let snapshot = ContextSnapshot::default();
        let rt = Runtime::new().unwrap();
        let outputs = rt.block_on(async { manager.render_all(&snapshot).await.unwrap() });
        assert!(outputs.is_empty());
    }
}
