use super::{CliContext, SyncArgs, SyncCommands};
use crate::core::loader;
use crate::error::PrismResult;
use crate::sync::auth;
use crate::sync::{SyncClient, SyncData};

pub fn handle_sync(args: SyncArgs, _ctx: &CliContext) -> PrismResult<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    match args.command {
        SyncCommands::Push => runtime.block_on(push()),
        SyncCommands::Pull => runtime.block_on(pull()),
        SyncCommands::Status => runtime.block_on(status()),
        SyncCommands::Configure => configure(),
    }
}

async fn push() -> PrismResult<()> {
    let themes = loader::list_available()?;
    let payload = SyncData {
        themes: themes
            .iter()
            .map(|entry| entry.metadata.name.clone())
            .collect(),
        config: serde_json::json!({"widgets": []}),
        dotfiles: Vec::new(),
        timestamp: chrono::Local::now().to_rfc3339(),
    };
    let client = SyncClient::new(None)?;
    client.push(payload).await
}

async fn pull() -> PrismResult<()> {
    let client = SyncClient::new(None)?;
    let payload = client.pull().await?;
    println!(
        "Downloaded {} themes from cloud (placeholder)",
        payload.themes.len()
    );
    Ok(())
}

async fn status() -> PrismResult<()> {
    let client = SyncClient::new(None)?;
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
    println!("Enter token via PRISM_SYNC_TOKEN env var or ~/.config/prism/auth");
    auth::write_token("demo-token")?;
    Ok(())
}
