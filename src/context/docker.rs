use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerContext {
    pub running: u32,
    pub total: u32,
    pub source: DockerSource,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DockerSource {
    #[default]
    Unknown,
    Env,
    Docker,
    Podman,
}

pub fn detect_docker_context() -> DockerContext {
    if let Some(ctx) = read_from_env() {
        return ctx;
    }
    if let Some(ctx) = query_binary("docker", DockerSource::Docker) {
        return ctx;
    }
    if let Some(ctx) = query_binary("podman", DockerSource::Podman) {
        return ctx;
    }
    DockerContext::default()
}

fn read_from_env() -> Option<DockerContext> {
    let value = std::env::var("PRISM_DOCKER_COUNT").ok()?;
    parse_value(&value, DockerSource::Env)
}

fn query_binary(binary: &str, source: DockerSource) -> Option<DockerContext> {
    let output = Command::new(binary)
        .args(["info", "--format", "{{.ContainersRunning}}/{{.Containers}}"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let raw = String::from_utf8_lossy(&output.stdout);
    parse_value(raw.trim(), source)
}

fn parse_value(raw: &str, source: DockerSource) -> Option<DockerContext> {
    let mut parts = raw.split('/');
    let running = parts
        .next()
        .and_then(|value| value.trim().parse::<u32>().ok())?;
    let total = parts
        .next()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .unwrap_or(running);
    Some(DockerContext {
        running,
        total,
        source,
    })
}
