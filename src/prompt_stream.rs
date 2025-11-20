use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

use crate::context::ContextSnapshot;
use crate::core::theme::Theme;
use crate::data_dir;
use crate::error::PrismResult;

pub const STREAM_FILE_NAME: &str = "prompt-stream.txt";

pub fn stream_path_from_config(config_dir: &Path) -> PathBuf {
    config_dir.join("data").join(STREAM_FILE_NAME)
}

pub fn stream_path() -> PrismResult<PathBuf> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir)?;
    Ok(dir.join(STREAM_FILE_NAME))
}

pub fn write_prompt(
    theme: &Theme,
    snapshot: &ContextSnapshot,
    widgets: &[String],
) -> PrismResult<String> {
    let line = compose_prompt(theme, snapshot, widgets);
    let path = stream_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, &line)?;
    Ok(line)
}

pub fn fallback_prompt(theme: &Theme) -> String {
    format!("[{}] › ", theme.metadata.name)
}

fn compose_prompt(theme: &Theme, snapshot: &ContextSnapshot, widgets: &[String]) -> String {
    let mut parts = Vec::new();
    let prompt = &theme.prompt;

    if prompt.show_user {
        parts.push(current_user());
    }
    if prompt.show_host {
        parts.push(current_host());
    }
    if prompt.show_time {
        parts.push(Local::now().format("%H:%M:%S").to_string());
    }
    if prompt.show_git {
        parts.push(format_git_segment(&snapshot.git));
    }

    let mut line = parts.join(&prompt.separator);
    let mut extras = Vec::new();
    if !widgets.is_empty() {
        extras.extend_from_slice(widgets);
    }
    for (name, segment) in prompt.ordered_segments() {
        let label = segment
            .icon
            .clone()
            .filter(|icon| !icon.trim().is_empty())
            .unwrap_or(name);
        extras.push(label);
    }
    if !extras.is_empty() {
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(&extras.join(" "));
    }
    line
}

fn current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "user".into())
}

fn current_host() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "host".into())
}

fn format_git_segment(ctx: &crate::context::git::GitContext) -> String {
    if let Some(branch) = &ctx.branch {
        let mut segment = format!(" {branch}");
        if ctx.dirty {
            segment.push('*');
        }
        if ctx.has_conflict {
            segment.push('!');
        }
        segment
    } else {
        " detatched".into()
    }
}
