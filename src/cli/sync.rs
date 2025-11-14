use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

use super::{
    CliContext, DotfileArgs, DotfileCommands, JwtArgs, JwtCommands, SyncArgs, SyncCommands,
};
use crate::core::loader;
use crate::error::{PrismError, PrismResult};
use crate::sync::auth;
use crate::sync::client::{DotfileRecord, SyncClient, SyncData};
use crate::sync::{dotfiles, history, jwt, state};
use crate::widgets::storage as widget_storage;
use crate::{ensure_config_dir, user_themes_dir};

const FORCE_ENV: &str = "PRISM_SYNC_FORCE";
const DOTFILES_ENV: &str = "PRISM_SYNC_DOTFILES";
const AUTO_RESTORE_ENV: &str = "PRISM_SYNC_AUTO_RESTORE";

pub fn handle_sync(args: SyncArgs, _ctx: &CliContext) -> PrismResult<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    match args.command {
        SyncCommands::Push => runtime.block_on(push()),
        SyncCommands::Pull => runtime.block_on(pull()),
        SyncCommands::Status => runtime.block_on(status()),
        SyncCommands::Configure => configure(),
        SyncCommands::History => show_history(),
        SyncCommands::Rollback => rollback(),
        SyncCommands::Dotfiles(args) => handle_dotfiles(args),
        SyncCommands::Jwt(args) => handle_jwt(args),
    }
}

async fn push() -> PrismResult<()> {
    let snapshot = build_snapshot()?;
    let mut state = state::load_state()?;
    let client = sync_client()?;
    let remote_status = client.status().await?;
    detect_conflict(&state, remote_status.remote_timestamp.clone())?;
    client.push(snapshot.clone()).await?;
    history::record_snapshot("push", &snapshot)?;
    let now = chrono::Local::now().to_rfc3339();
    state.last_push = Some(now);
    state.last_remote = Some(snapshot.timestamp.clone());
    state::save_state(&state)?;
    println!(
        "Pushed {} themes and {} dotfiles",
        snapshot.themes.len(),
        snapshot.dotfiles.len()
    );
    Ok(())
}

async fn pull() -> PrismResult<()> {
    let client = sync_client()?;
    let payload = client.pull().await?;
    apply_snapshot(&payload)?;
    history::record_snapshot("pull", &payload)?;
    let mut state = state::load_state()?;
    let now = chrono::Local::now().to_rfc3339();
    state.last_pull = Some(now);
    state.last_remote = Some(payload.timestamp.clone());
    state::save_state(&state)?;
    println!(
        "Pulled {} themes, {} dotfiles",
        payload.themes.len(),
        payload.dotfiles.len()
    );
    Ok(())
}

async fn status() -> PrismResult<()> {
    let client = sync_client()?;
    let status = client.status().await?;
    println!("Local: {}", status.local_timestamp);
    if let Some(remote) = status.remote_timestamp {
        println!("Remote: {remote}");
    } else {
        println!("Remote status unavailable");
    }
    Ok(())
}

fn configure() -> PrismResult<()> {
    if let Ok(token) = std::env::var("PRISM_SYNC_TOKEN") {
        auth::write_token(token.trim())?;
        println!("Stored sync token from PRISM_SYNC_TOKEN");
    }
    if let Ok(secret) = std::env::var("PRISM_SYNC_JWT_SECRET") {
        auth::write_jwt_secret(secret.trim())?;
        println!("Stored JWT secret from PRISM_SYNC_JWT_SECRET");
    }
    if auth::resolve_token()?.is_none() {
        println!("Set PRISM_SYNC_TOKEN and rerun `prism sync configure` to persist it.");
    }
    Ok(())
}

fn show_history() -> PrismResult<()> {
    let entries = history::list()?;
    if entries.is_empty() {
        println!("No sync history recorded yet.");
    } else {
        println!("Sync history (oldest → newest):");
        for entry in entries {
            println!(
                "- {} :: {} ({})",
                entry.timestamp, entry.action, entry.snapshot_path
            );
        }
    }
    Ok(())
}

fn rollback() -> PrismResult<()> {
    if let Some(entry) = history::latest()? {
        let payload = history::load_snapshot(&entry.snapshot_path)?;
        apply_snapshot(&payload)?;
        println!(
            "Rolled back to {} snapshot captured at {}",
            entry.action, entry.timestamp
        );
    } else {
        println!("No snapshots available for rollback.");
    }
    Ok(())
}

fn handle_dotfiles(args: DotfileArgs) -> PrismResult<()> {
    match args.command {
        DotfileCommands::List => {
            let files = dotfiles::tracked_files()?;
            if files.is_empty() {
                println!("No tracked dotfiles under ~/.config/prism/dotfiles");
            } else {
                println!("Tracked dotfiles:");
                for path in files {
                    match dotfiles::describe(&path) {
                        Ok(info) => {
                            let mut summary = format!("{} bytes", info.size);
                            if let Some(modified) = info.modified.as_deref() {
                                summary.push_str(&format!(", modified {modified}"));
                            }
                            let digest = &info.sha256;
                            let short = if digest.len() > 12 {
                                &digest[..12]
                            } else {
                                digest.as_str()
                            };
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                println!("- {name} ({summary}, sha {short})");
                            }
                        }
                        Err(_) => {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                println!("- {name}");
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        DotfileCommands::Restore { name, destination } => restore_dotfile(&name, destination),
    }
}

fn handle_jwt(args: JwtArgs) -> PrismResult<()> {
    match args.command {
        JwtCommands::Issue {
            subject,
            ttl,
            secret,
            no_store,
        } => issue_jwt(&subject, ttl, secret, no_store),
    }
}

fn restore_dotfile(name: &str, destination: Option<PathBuf>) -> PrismResult<()> {
    let source = dotfiles_root()?.join(name);
    if !source.exists() {
        return Err(PrismError::new(format!("dotfile '{name}' not found")));
    }
    let target = match destination {
        Some(path) => finalize_destination(path, name),
        None => {
            let home = dirs::home_dir().ok_or_else(|| PrismError::new("missing home directory"))?;
            home.join(name)
        }
    };
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(&source, &target)?;
    println!("Restored {name} to {}", target.display());
    Ok(())
}

fn finalize_destination(path: PathBuf, name: &str) -> PathBuf {
    if path.is_dir() {
        path.join(name)
    } else {
        path
    }
}

fn sync_client() -> PrismResult<SyncClient> {
    let token = auth::resolve_token()?;
    let secret = auth::resolve_jwt_secret()?;
    SyncClient::new(None, token, secret)
}

fn build_snapshot() -> PrismResult<SyncData> {
    let themes = loader::list_available()?;
    let config_dir = ensure_config_dir()?;
    let enabled = widget_storage::load_enabled(&config_dir)?;
    let settings = widget_storage::load_settings(&config_dir)?;
    let dotfiles = dotfiles::tracked_files()?
        .into_iter()
        .filter_map(|path| match dotfiles::describe(&path) {
            Ok(info) => {
                let encoded = BASE64.encode(&info.contents);
                Some(DotfileRecord {
                    name: info.name,
                    original: Some(path.display().to_string()),
                    contents: encoded,
                    size: Some(info.size),
                    modified: info.modified,
                    sha256: Some(info.sha256),
                    permissions: info.permissions,
                })
            }
            Err(err) => {
                log::warn!("skipping dotfile {}: {err}", path.display());
                None
            }
        })
        .collect();

    Ok(SyncData {
        themes: themes
            .iter()
            .map(|entry| entry.metadata.name.clone())
            .collect(),
        config: serde_json::json!({
            "widgets": enabled,
            "widget_settings": settings
        }),
        dotfiles,
        timestamp: chrono::Local::now().to_rfc3339(),
    })
}

fn apply_snapshot(snapshot: &SyncData) -> PrismResult<()> {
    let config_dir = ensure_config_dir()?;
    if let Some(entries) = snapshot.config.get("widgets").and_then(|v| v.as_array()) {
        let enabled = entries
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<_>>();
        widget_storage::save_enabled(&config_dir, &enabled)?;
    }
    if let Some(settings) = snapshot
        .config
        .get("widget_settings")
        .and_then(|v| v.as_object())
    {
        let mut map = BTreeMap::new();
        for (widget, value) in settings {
            if let Some(obj) = value.as_object() {
                let mut inner = BTreeMap::new();
                for (key, val) in obj {
                    if let Some(text) = val.as_str() {
                        inner.insert(key.clone(), text.into());
                    }
                }
                map.insert(widget.clone(), inner);
            }
        }
        widget_storage::save_settings(&config_dir, &map)?;
    }

    let user_dir = user_themes_dir()?;
    for theme_name in &snapshot.themes {
        if let Ok(theme) = loader::load_theme_by_name(theme_name) {
            let dest = user_dir.join(format!("{theme_name}.toml"));
            let serialized = toml::to_string_pretty(&theme)?;
            fs::write(dest, serialized)?;
        }
    }

    let root = dotfiles_root()?;
    fs::create_dir_all(&root)?;
    for record in selected_dotfiles(snapshot) {
        let bytes = BASE64
            .decode(&record.contents)
            .map_err(|err| PrismError::new(format!("invalid dotfile data: {err}")))?;
        if let Some(expected) = record.sha256.as_deref() {
            let actual = dotfiles::hash_contents(&bytes);
            if actual != expected {
                return Err(PrismError::new(format!(
                    "dotfile {} failed integrity check",
                    record.name
                )));
            }
        }
        let dest = root.join(&record.name);
        fs::write(&dest, &bytes)?;
        if let Some(mode) = record.permissions.as_deref() {
            apply_permissions(&dest, mode);
        }
        if std::env::var(AUTO_RESTORE_ENV).ok().as_deref() == Some("1") {
            let target = record
                .original
                .as_deref()
                .map(PathBuf::from)
                .or_else(|| dirs::home_dir().map(|home| home.join(&record.name)))
                .unwrap_or(dest.clone());
            if let Some(parent) = target.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::copy(&dest, &target);
            if let Some(mode) = record.permissions.as_deref() {
                apply_permissions(&target, mode);
            }
        }
    }
    Ok(())
}

fn selected_dotfiles(snapshot: &SyncData) -> Vec<&DotfileRecord> {
    match std::env::var(DOTFILES_ENV) {
        Ok(value) if value.eq_ignore_ascii_case("none") => Vec::new(),
        Ok(value) if value.eq_ignore_ascii_case("all") || value.trim().is_empty() => {
            snapshot.dotfiles.iter().collect()
        }
        Ok(list) => {
            let wanted: HashSet<_> = list
                .split(',')
                .map(|entry| entry.trim().to_string())
                .collect();
            snapshot
                .dotfiles
                .iter()
                .filter(|record| wanted.contains(&record.name))
                .collect()
        }
        Err(_) => snapshot.dotfiles.iter().collect(),
    }
}

fn detect_conflict(state: &state::SyncState, remote: Option<String>) -> PrismResult<()> {
    let force = std::env::var(FORCE_ENV).is_ok();
    if force {
        return Ok(());
    }
    if let Some(remote) = remote {
        if let Some(last_remote) = &state.last_remote {
            if last_remote != &remote {
                return Err(PrismError::new(
                    "Remote changes detected. Pull or set PRISM_SYNC_FORCE=1 to override.",
                ));
            }
        } else {
            return Err(PrismError::new(
                "No sync history found. Pull before pushing or set PRISM_SYNC_FORCE=1.",
            ));
        }
    }
    Ok(())
}

fn dotfiles_root() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join("dotfiles"))
}

fn issue_jwt(
    subject: &str,
    ttl_secs: u64,
    secret: Option<String>,
    no_store: bool,
) -> PrismResult<()> {
    if ttl_secs == 0 {
        return Err(PrismError::new("Token lifetime must be at least 1 second"));
    }
    let ttl = i64::try_from(ttl_secs).map_err(|_| PrismError::new("TTL value too large"))?;
    let duration = chrono::Duration::seconds(ttl);
    let secret = match secret {
        Some(value) => value,
        None => auth::resolve_jwt_secret()?.ok_or_else(|| {
            PrismError::new("No JWT secret found. Provide --secret or set PRISM_SYNC_JWT_SECRET.")
        })?,
    };
    let token = jwt::issue(&secret, Some(subject), duration)?;
    let expires_at = (chrono::Utc::now() + duration).to_rfc3339();
    if !no_store {
        auth::write_token(&token)?;
        if let Ok(path) = auth::auth_path() {
            println!("Stored JWT token at {}", path.display());
        }
    }
    println!("Issued JWT for subject '{subject}' (expires {expires_at})");
    println!("{token}");
    Ok(())
}

fn apply_permissions(path: &Path, mode: &str) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(value) = u32::from_str_radix(mode, 8) {
            if let Ok(metadata) = fs::metadata(path) {
                let mut perms = metadata.permissions();
                perms.set_mode(value);
                if let Err(err) = fs::set_permissions(path, perms) {
                    log::warn!("unable to apply permissions on {}: {err}", path.display());
                }
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        let _ = mode;
    }
}
