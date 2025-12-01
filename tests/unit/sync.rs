use std::net::SocketAddr;

use chrono::{Duration, Utc};
use prism::error::PrismResult;
use prism::sync::client::{ClientOptions, DotfileRecord, SyncClient, SyncData};
use prism::sync::jwt;
use prism::sync::server::serve_with_listener;
use serde_json::json;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use super::helpers::TestEnv;

#[tokio::test]
async fn sync_client_push_and_pull_round_trip() -> PrismResult<()> {
    let _env = TestEnv::new();
    let secret = "unit-sync-secret";
    let token = jwt::issue(secret, Some("tester"), Duration::minutes(5))?;
    let server = spawn_sync_server(secret).await?;
    let client = build_client(server.addr(), &token, secret)?;

    let initial_status = client.status().await?;
    assert!(initial_status.remote_version.is_none());

    let payload = payload_with_prompt("default");
    let push = client.push(payload.clone(), None).await?;
    assert_eq!(push.version, 1);

    let pulled = client.pull().await?;
    assert_eq!(pulled.themes, payload.themes);
    assert_eq!(pulled.config, payload.config);
    assert_eq!(pulled.dotfiles.len(), payload.dotfiles.len());
    assert_eq!(pulled.version, Some(push.version));

    let status_after = client.status().await?;
    assert_eq!(status_after.remote_version, Some(push.version));
    assert!(status_after.remote_timestamp.is_some());

    server.shutdown().await
}

#[tokio::test]
async fn sync_client_rejects_conflicting_pushes() -> PrismResult<()> {
    let _env = TestEnv::new();
    let secret = "unit-sync-secret";
    let token = jwt::issue(secret, Some("tester"), Duration::minutes(5))?;
    let server = spawn_sync_server(secret).await?;
    let client = build_client(server.addr(), &token, secret)?;

    let base_payload = payload_with_prompt("initial");
    let first = client.push(base_payload.clone(), None).await?;
    assert_eq!(first.version, 1);

    let remote_payload = payload_with_prompt("remote");
    let remote = client
        .push(remote_payload, Some(first.version))
        .await
        .expect("remote update should succeed");
    assert_eq!(remote.version, 2);

    let stale_payload = payload_with_prompt("local");
    let err = client
        .push(stale_payload, Some(first.version))
        .await
        .expect_err("stale push should fail with conflict");
    let message = err.to_string();
    assert!(
        message.contains("Conflicting fields")
            || message.contains("conflict")
            || message.contains("Missing base_version"),
        "expected conflict message, got {message}"
    );

    server.shutdown().await
}

struct SpawnedServer {
    addr: SocketAddr,
    shutdown_tx: oneshot::Sender<()>,
    handle: JoinHandle<PrismResult<()>>,
}

impl SpawnedServer {
    fn addr(&self) -> SocketAddr {
        self.addr
    }

    async fn shutdown(self) -> PrismResult<()> {
        let _ = self.shutdown_tx.send(());
        self.handle.await.expect("server join")?;
        Ok(())
    }
}

async fn spawn_sync_server(secret: &str) -> PrismResult<SpawnedServer> {
    std::env::set_var("PRISM_SYNC_DISABLE_PERSIST", "1");
    let _ = prism::sync::server::reset_backend_store();
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let secret = secret.to_string();
    let handle = tokio::spawn(async move {
        serve_with_listener(listener, secret, async move {
            let _ = shutdown_rx.await;
        })
        .await
    });
    Ok(SpawnedServer {
        addr,
        shutdown_tx,
        handle,
    })
}

fn build_client(addr: SocketAddr, token: &str, secret: &str) -> PrismResult<SyncClient> {
    SyncClient::new(
        ClientOptions {
            endpoint: Some(format!("http://{}", addr)),
            ..Default::default()
        },
        Some(token.to_string()),
        Some(secret.to_string()),
    )
}

fn payload_with_prompt(prompt: &str) -> SyncData {
    SyncData {
        themes: vec!["Solstice".into()],
        config: json!({ "prompt": prompt }),
        dotfiles: vec![sample_dotfile()],
        timestamp: Utc::now().to_rfc3339(),
        version: None,
    }
}

fn sample_dotfile() -> DotfileRecord {
    DotfileRecord {
        name: "prism.zsh".into(),
        original: Some("~/.zshrc".into()),
        contents: "export PROMPT='%F{cyan}IRIDEX%f '".to_string(),
        size: Some(32),
        modified: None,
        sha256: None,
        permissions: Some("0644".into()),
    }
}
