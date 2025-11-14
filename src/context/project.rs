use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectContext {
    pub language: Option<String>,
    pub manifest: Option<String>,
}

pub fn detect_project_context(path: &Path) -> ProjectContext {
    let mut ctx = ProjectContext::default();
    if path.join("Cargo.toml").exists() {
        ctx.language = Some("rust".into());
        ctx.manifest = Some(path.join("Cargo.toml").display().to_string());
    } else if path.join("package.json").exists() {
        ctx.language = Some("javascript".into());
        ctx.manifest = Some(path.join("package.json").display().to_string());
    } else if path.join("requirements.txt").exists() {
        ctx.language = Some("python".into());
        ctx.manifest = Some(path.join("requirements.txt").display().to_string());
    } else if path.join("go.mod").exists() {
        ctx.language = Some("go".into());
        ctx.manifest = Some(path.join("go.mod").display().to_string());
    }
    ctx
}
