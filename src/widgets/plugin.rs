use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::timeout;

use crate::context::ContextSnapshot;
use crate::error::{PrismError, PrismResult};

use super::widget::{Widget, WidgetOutput};

#[derive(Debug, Clone, Deserialize)]
pub struct PluginDescriptor {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub interval_ms: Option<u64>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

pub struct PluginRegistry {
    plugins: BTreeMap<String, PluginDescriptor>,
}

impl PluginRegistry {
    pub fn load(config_dir: &Path) -> PrismResult<Self> {
        let dir = config_dir.join("plugins").join("widgets");
        let mut plugins = BTreeMap::new();
        if dir.exists() {
            for entry in std::fs::read_dir(&dir)? {
                let entry = entry?;
                if !entry.file_type()?.is_file() {
                    continue;
                }
                let raw = std::fs::read_to_string(entry.path())?;
                match toml::from_str::<PluginDescriptor>(&raw) {
                    Ok(descriptor) => {
                        plugins.insert(descriptor.name.clone(), descriptor);
                    }
                    Err(err) => {
                        log::warn!("Failed to parse widget plugin {:?}: {err}", entry.path());
                    }
                }
            }
        }
        Ok(Self { plugins })
    }

    pub fn instantiate(&self, name: &str) -> Option<PluginWidget> {
        self.plugins.get(name).cloned().map(PluginWidget::new)
    }
}

pub struct PluginWidget {
    descriptor: PluginDescriptor,
}

impl PluginWidget {
    fn new(descriptor: PluginDescriptor) -> Self {
        Self { descriptor }
    }

    fn interval(&self) -> Duration {
        Duration::from_millis(self.descriptor.interval_ms.unwrap_or(1500))
    }

    fn timeout(&self) -> Duration {
        Duration::from_millis(self.descriptor.timeout_ms.unwrap_or(1500))
    }
}

#[async_trait::async_trait]
impl Widget for PluginWidget {
    fn name(&self) -> &str {
        &self.descriptor.name
    }

    fn refresh_interval(&self) -> Duration {
        self.interval()
    }

    async fn render(&mut self, snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
        let mut command = Command::new(&self.descriptor.command);
        command.args(&self.descriptor.args);
        command.envs(&self.descriptor.env);
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        let mut child = command.spawn().map_err(|err| {
            PrismError::new(format!("Plugin {} failed to spawn: {err}", self.name()))
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            let payload = serde_json::to_vec(&PluginInput::from_snapshot(snapshot))?;
            stdin.write_all(&payload).await.map_err(|err| {
                PrismError::new(format!("Plugin {} stdin error: {err}", self.name()))
            })?;
        }

        let output = timeout(self.timeout(), child.wait_with_output())
            .await
            .map_err(|_| PrismError::new(format!("Plugin {} timed out", self.name())))?
            .map_err(|err| PrismError::new(format!("Plugin {} failed: {err}", self.name())))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PrismError::new(format!(
                "Plugin {} exited with {}: {stderr}",
                self.name(),
                output.status
            )));
        }

        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() {
            Ok(WidgetOutput::static_text(format!(
                "{}: (no output)",
                self.name()
            )))
        } else {
            Ok(WidgetOutput::static_text(text))
        }
    }
}

#[derive(Debug, Serialize)]
struct PluginInput {
    git: PluginGitContext,
    project_language: Option<String>,
    time_period: String,
    system_load: f32,
    docker_running: u32,
    docker_total: u32,
}

#[derive(Debug, Serialize)]
struct PluginGitContext {
    branch: Option<String>,
    dirty: bool,
    ahead: usize,
    behind: usize,
    conflicts: bool,
}

impl PluginInput {
    fn from_snapshot(snapshot: &ContextSnapshot) -> Self {
        Self {
            git: PluginGitContext {
                branch: snapshot.git.branch.clone(),
                dirty: snapshot.git.dirty,
                ahead: snapshot.git.ahead,
                behind: snapshot.git.behind,
                conflicts: snapshot.git.has_conflict,
            },
            project_language: snapshot.project.language.clone(),
            time_period: snapshot.time.period.clone(),
            system_load: snapshot.system.load_percent,
            docker_running: snapshot.docker.running,
            docker_total: snapshot.docker.total,
        }
    }
}
