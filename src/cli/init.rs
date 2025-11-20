use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::Local;
use colored::Colorize;
use regex::Regex;

use crate::cli::{InitArgs, ApplyArgs};
use crate::catalog::load_catalog;
use crate::error::{PrismError, PrismResult};
use crate::{ensure_config_dir, user_themes_dir};

pub fn handle_init(args: InitArgs) -> PrismResult<()> {
    if args.undo {
        return handle_undo(args.purge);
    }

    // 1. Ensure config dirs
    let _config_dir = ensure_config_dir()?;
    let _user_dir = user_themes_dir()?;
    println!("{} {}", "✔".green(), "Configuration directories ready.");

    // 2. Detect shell and RC file
    let shell = detect_shell()?;
    let rc_file = get_rc_file(&shell)?;
    println!("{} Detected shell: {} -> {}", "✔".green(), shell, rc_file.display());

    // 3. Backup RC file
    if rc_file.exists() {
        backup_rc_file(&rc_file)?;
    } else {
        // If it doesn't exist, we create it (empty)
        if let Some(parent) = rc_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(&rc_file)?;
        println!("{} Created new RC file: {}", "✔".green(), rc_file.display());
    }

    // 4. Append Prism block
    append_prism_block(&rc_file, &shell)?;

    // 5. Apply theme if requested
    if let Some(theme_name) = args.theme {
        let catalog = load_catalog()?;
        crate::cli::handle_apply(&catalog, ApplyArgs { theme: theme_name })?;
    } else {
        println!("\n{}", "Prism initialized successfully!".bold().green());
        println!("Run {} to select a theme.", "prism apply <theme>".cyan());
    }

    Ok(())
}

fn handle_undo(purge: bool) -> PrismResult<()> {
    let shell = detect_shell()?;
    let rc_file = get_rc_file(&shell)?;
    
    if rc_file.exists() {
        remove_prism_block(&rc_file)?;
        println!("{} Removed Prism block from {}", "✔".green(), rc_file.display());
    }

    if purge {
        if let Ok(config_dir) = crate::ensure_config_dir() {
             // Safety check: ensure we are deleting the prism config dir
             if config_dir.ends_with("prism") {
                 fs::remove_dir_all(&config_dir)?;
                 println!("{} Purged configuration directory: {}", "✔".green(), config_dir.display());
             }
        }
    }

    Ok(())
}

fn detect_shell() -> PrismResult<String> {
    let shell_env = std::env::var("SHELL").map_err(|_| PrismError::new("Could not detect $SHELL"))?;
    let shell_name = Path::new(&shell_env)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| PrismError::new("Invalid $SHELL path"))?;
    
    match shell_name {
        "zsh" | "bash" | "fish" => Ok(shell_name.to_string()),
        _ => Err(PrismError::new(format!("Unsupported shell: {}", shell_name))),
    }
}

fn get_rc_file(shell: &str) -> PrismResult<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| PrismError::new("Could not determine home directory"))?;
    match shell {
        "zsh" => Ok(home.join(".zshrc")),
        "bash" => Ok(home.join(".bashrc")),
        "fish" => Ok(home.join(".config/fish/config.fish")),
        _ => Err(PrismError::new(format!("Unsupported shell: {}", shell))),
    }
}

fn backup_rc_file(path: &Path) -> PrismResult<()> {
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let filename = path.file_name().unwrap().to_string_lossy();
    let backup_path = path.with_file_name(format!("{}.prism-backup-{}", filename, timestamp));
    
    fs::copy(path, &backup_path)?;
    println!("{} Created backup: {}", "✔".green(), backup_path.display());
    Ok(())
}

fn append_prism_block(path: &Path, shell: &str) -> PrismResult<()> {
    let content = fs::read_to_string(path).unwrap_or_default();
    if content.contains("# Prism Terminal") {
        println!("{} Prism block already exists in {}", "ℹ".yellow(), path.display());
        return Ok(());
    }

    let mut file = fs::OpenOptions::new().append(true).open(path)?;
    
    let block = match shell {
        "fish" => format!(
            "\n# Prism Terminal\nif test -f \"$HOME/.config/prism/prism.fish\"\n    source \"$HOME/.config/prism/prism.fish\"\nend\n"
        ),
        _ => format!(
            "\n# Prism Terminal\nif [ -f \"$HOME/.config/prism/prism.{}\" ]; then\n    source \"$HOME/.config/prism/prism.{}\"\nfi\n",
            shell, shell
        ),
    };

    file.write_all(block.as_bytes())?;
    println!("{} Added Prism block to {}", "✔".green(), path.display());
    Ok(())
}

fn remove_prism_block(path: &Path) -> PrismResult<()> {
    let content = fs::read_to_string(path)?;
    if !content.contains("# Prism Terminal") {
        return Ok(());
    }

    // Regex to match the block. 
    // We need to be careful to match the exact block we added.
    // The block starts with "# Prism Terminal" and ends with "fi" or "end".
    // We use (?s) to enable dot-matches-newline.
    let re = Regex::new(r"(?s)\n?# Prism Terminal.*?(?:fi|end)\n").map_err(|e| PrismError::new(e.to_string()))?;
    let new_content = re.replace(&content, "");
    
    fs::write(path, new_content.as_bytes())?;
    Ok(())
}
