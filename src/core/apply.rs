use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::core::prompt;
use crate::core::theme::Theme;
use crate::error::{PrismError, PrismResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
}

impl Shell {
    pub fn extension(&self) -> &'static str {
        match self {
            Shell::Zsh => "zsh",
            Shell::Bash => "bash",
            Shell::Fish => "fish",
        }
    }

    pub fn rc_file(&self) -> PrismResult<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| PrismError::new("missing home directory"))?;
        let file = match self {
            Shell::Zsh => home.join(".zshrc"),
            Shell::Bash => home.join(".bashrc"),
            Shell::Fish => home.join(".config/fish/config.fish"),
        };
        Ok(file)
    }
}

pub fn apply_theme(theme: &Theme, shell: Shell, config_dir: &Path) -> PrismResult<PathBuf> {
    fs::create_dir_all(config_dir)?;
    let script_path = config_dir.join(format!("prism.{}", shell.extension()));
    backup_if_exists(&script_path)?;
    let script = match shell {
        Shell::Zsh => prompt::generate_zsh(theme),
        Shell::Bash => prompt::generate_bash(theme),
        Shell::Fish => prompt::generate_fish(theme),
    };
    fs::write(&script_path, script)?;
    ensure_shell_hook(shell, &script_path)?;
    Ok(script_path)
}

pub fn revert(shell: Shell, config_dir: &Path) -> PrismResult<()> {
    let script_path = config_dir.join(format!("prism.{}", shell.extension()));
    if script_path.exists() {
        fs::remove_file(script_path)?;
    }
    Ok(())
}

fn backup_if_exists(path: &Path) -> PrismResult<()> {
    if path.exists() {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let backup = path.with_extension(format!("bak-{ts}"));
        fs::copy(path, backup)?;
    }
    Ok(())
}

fn ensure_shell_hook(shell: Shell, script_path: &Path) -> PrismResult<()> {
    let rc_file = shell.rc_file()?;
    if let Some(parent) = rc_file.parent() {
        fs::create_dir_all(parent)?;
    }
    let hook_line = format!("source {}\n", script_path.display());
    if rc_file.exists() {
        let contents = fs::read_to_string(&rc_file)?;
        if !contents.contains(&hook_line) {
            let mut new_contents = contents;
            if !new_contents.ends_with('\n') {
                new_contents.push('\n');
            }
            new_contents.push_str(&hook_line);
            fs::write(rc_file, new_contents)?;
        }
    } else {
        fs::write(rc_file, &hook_line)?;
    }
    Ok(())
}
