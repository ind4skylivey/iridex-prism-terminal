use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use chrono::Utc;
use prism::context::rules;
use prism::context::ContextDetector;
use prism::core::apply::Shell;
use prism::core::loader;
use prism::core::theme::Theme;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn with_isolated_env(test: impl FnOnce()) {
    // use a re-entrant mutex to avoid poisoning across tests if one fails?
    // actually just handling the poison error is enough for test isolation
    let lock = TEST_MUTEX.get_or_init(|| Mutex::new(()));
    let _guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let dir = tempfile::tempdir().expect("config dir");
    std::env::set_var("PRISM_CONFIG_DIR", dir.path());
    std::env::set_var("PRISM_DISABLE_SHELL_HOOKS", "1");
    test();
    std::env::remove_var("PRISM_CONFIG_DIR");
    std::env::remove_var("PRISM_DISABLE_SHELL_HOOKS");
}

fn make_dotfile(name: &str, contents: &str) -> prism::sync::client::DotfileRecord {
    prism::sync::client::DotfileRecord {
        name: name.into(),
        original: None,
        contents: BASE64.encode(contents.as_bytes()),
        size: Some(contents.len() as u64),
        modified: Some(Utc::now().to_rfc3339()),
        sha256: Some(prism::sync::dotfiles::hash_contents(contents.as_bytes())),
        permissions: None,
    }
}

#[test]
fn load_builtin_themes() {
    with_isolated_env(|| {
        // Filter out themes that are just raw palettes (JSON) and not full themes
        // The loader::list_available() includes palettes as themes, but Theme::load expects full TOML themes structure
        // unless we updated Theme::load to handle palette JSONs by converting them.
        // However, since we just want to test stability here:
        let themes = loader::list_available().expect("themes");
        assert!(!themes.is_empty());
        for entry in themes {
            // Only attempt to load/validate TOML themes in this integration test
            // Raw JSON palettes are handled by catalog loader which converts them to themes internally
            if entry.path.extension().map(|s| s == "toml").unwrap_or(false) {
               let theme = Theme::load(&entry.path).expect("load theme");
               theme.validate().expect("validate");
            }
        }
    });
}

#[test]
fn context_detector_runs() {
    with_isolated_env(|| {
        let detector = ContextDetector::new(
            rules::load_rules().expect("rules"),
            rules::load_manual_override().expect("override"),
        );
        let snapshot = detector.detect(Path::new("."));
        assert!(snapshot.is_ok());
    });
}

#[test]
fn apply_is_dry_run_ready() {
    with_isolated_env(|| {
        let themes = loader::list_available().expect("themes");
        let theme = Theme::load(&themes[0].path).expect("theme");
        let dir = tempfile::tempdir().expect("tmp");
        prism::core::apply::apply_theme(&theme, Shell::Zsh, dir.path()).expect("apply");
    });
}

#[test]
fn multi_shell_apply_generates_scripts() {
    with_isolated_env(|| {
        let dir = tempfile::tempdir().expect("tmp");
        let theme_entry = loader::list_available().expect("themes")[0].clone();
        let theme = Theme::load(&theme_entry.path).expect("theme");
        for shell in [Shell::Zsh, Shell::Bash, Shell::Fish] {
            let path = prism::core::apply::apply_theme(&theme, shell, dir.path()).expect("apply");
            assert!(path.exists());
        }
    });
}

#[test]
fn sync_server_roundtrip() {
    with_isolated_env(|| {
        std::env::set_var("PRISM_SYNC_JWT_SECRET", "test-secret");
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("listener");
            let addr = listener.local_addr().expect("addr");
            let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
            let secret = "test-secret".to_string();
            let server = tokio::spawn(async move {
                let _ = prism::sync::server::serve_with_listener(listener, secret, async {
                    let _ = shutdown_rx.await;
                })
                .await;
            });

            let token = prism::sync::jwt::issue(
                "test-secret",
                Some("tester"),
                chrono::Duration::seconds(120),
            )
            .expect("token");
            let endpoint = format!("http://{}", addr);
            let client = prism::sync::SyncClient::new(
                prism::sync::client::ClientOptions::from(Some(endpoint.clone())),
                Some(token),
                Some("test-secret".into()),
            )
            .expect("client");

            let payload = prism::sync::client::SyncData {
                themes: vec!["cyberpunk".into()],
                config: serde_json::json!({"widgets": []}),
                dotfiles: vec![prism::sync::client::DotfileRecord {
                    name: "test-file".into(),
                    original: None,
                    contents: BASE64.encode(b"hello"),
                    size: Some(5),
                    modified: Some(Utc::now().to_rfc3339()),
                    sha256: Some(prism::sync::dotfiles::hash_contents(b"hello")),
                    permissions: None,
                }],
                timestamp: Utc::now().to_rfc3339(),
                version: None,
            };

            let response = client.push(payload.clone(), None).await.expect("push");
            assert_eq!(response.version, 1);
            let pulled = client.pull().await.expect("pull");
            assert_eq!(pulled.themes, payload.themes);
            assert_eq!(pulled.dotfiles.len(), 1);
            let status = client.status().await.expect("status");
            assert!(status.remote_timestamp.is_some());

            let _ = shutdown_tx.send(());
            let _ = server.await;
        });
        std::env::remove_var("PRISM_SYNC_JWT_SECRET");
    });
}

#[test]
fn sync_status_requires_authentication() {
    with_isolated_env(|| {
        std::env::set_var("PRISM_SYNC_JWT_SECRET", "test-secret");
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("listener");
            let addr = listener.local_addr().expect("addr");
            let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
            let secret = "test-secret".to_string();
            let server = tokio::spawn(async move {
                let _ = prism::sync::server::serve_with_listener(listener, secret, async {
                    let _ = shutdown_rx.await;
                })
                .await;
            });

            let endpoint = format!("http://{}", addr);
            let client = prism::sync::SyncClient::new(
                prism::sync::client::ClientOptions::from(Some(endpoint)),
                None,
                Some("test-secret".into()),
            )
            .expect("client");

            let status = client
                .status()
                .await
                .expect("status without token should still return fallback");
            assert!(status.remote_timestamp.is_none());
            let error = status
                .remote_error
                .expect("expected remote error when unauthorized");
            assert!(
                error.contains("401"),
                "expected HTTP 401 in error message, got {error}"
            );

            let _ = shutdown_tx.send(());
            let _ = server.await;
        });
        std::env::remove_var("PRISM_SYNC_JWT_SECRET");
    });
}

#[test]
fn sync_merges_divergent_pushes() {
    with_isolated_env(|| {
        std::env::set_var("PRISM_SYNC_JWT_SECRET", "test-secret");
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("listener");
            let addr = listener.local_addr().expect("addr");
            let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
            let secret = "test-secret".to_string();
            let server = tokio::spawn(async move {
                let _ = prism::sync::server::serve_with_listener(listener, secret, async {
                    let _ = shutdown_rx.await;
                })
                .await;
            });

            let token = prism::sync::jwt::issue(
                "test-secret",
                Some("tester"),
                chrono::Duration::seconds(120),
            )
            .expect("token");
            let endpoint = format!("http://{}", addr);
            let client = prism::sync::SyncClient::new(
                prism::sync::client::ClientOptions::from(Some(endpoint.clone())),
                Some(token),
                Some("test-secret".into()),
            )
            .expect("client");

            let base_payload = prism::sync::client::SyncData {
                themes: vec!["cyberpunk".into()],
                config: serde_json::json!({"widgets": []}),
                dotfiles: vec![make_dotfile("alpha", "base")],
                timestamp: Utc::now().to_rfc3339(),
                version: None,
            };

            let resp1 = client
                .push(base_payload.clone(), None)
                .await
                .expect("initial push");
            assert_eq!(resp1.version, 1);

            let mut host_a_payload = base_payload.clone();
            host_a_payload.dotfiles = vec![make_dotfile("alpha", "host-a")];
            let resp2 = client
                .push(host_a_payload, Some(resp1.version))
                .await
                .expect("host a push");
            assert_eq!(resp2.version, 2);

            let mut host_b_payload = base_payload.clone();
            host_b_payload.dotfiles = vec![
                make_dotfile("alpha", "base"),
                make_dotfile("beta", "host-b"),
            ];
            let resp3 = client
                .push(host_b_payload, Some(resp1.version))
                .await
                .expect("host b push");
            assert_eq!(resp3.version, 3);

            let merged = client.pull().await.expect("pull");
            assert_eq!(merged.dotfiles.len(), 2);
            let mut map = BTreeMap::new();
            for record in merged.dotfiles {
                let bytes = BASE64.decode(&record.contents).expect("decode");
                map.insert(record.name.clone(), String::from_utf8(bytes).expect("utf8"));
            }
            assert_eq!(map.get("alpha"), Some(&"host-a".to_string()));
            assert_eq!(map.get("beta"), Some(&"host-b".to_string()));

            let _ = shutdown_tx.send(());
            let _ = server.await;
        });
        std::env::remove_var("PRISM_SYNC_JWT_SECRET");
    });
}

#[test]
fn sync_conflict_on_same_dotfile() {
    with_isolated_env(|| {
        std::env::set_var("PRISM_SYNC_JWT_SECRET", "test-secret");
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("listener");
            let addr = listener.local_addr().expect("addr");
            let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
            let secret = "test-secret".to_string();
            let server = tokio::spawn(async move {
                let _ = prism::sync::server::serve_with_listener(listener, secret, async {
                    let _ = shutdown_rx.await;
                })
                .await;
            });

            let token = prism::sync::jwt::issue(
                "test-secret",
                Some("tester"),
                chrono::Duration::seconds(120),
            )
            .expect("token");
            let endpoint = format!("http://{}", addr);
            let client = prism::sync::SyncClient::new(
                prism::sync::client::ClientOptions::from(Some(endpoint)),
                Some(token),
                Some("test-secret".into()),
            )
            .expect("client");

            let base_payload = prism::sync::client::SyncData {
                themes: vec!["cyberpunk".into()],
                config: serde_json::json!({"widgets": []}),
                dotfiles: vec![make_dotfile("alpha", "seed")],
                timestamp: Utc::now().to_rfc3339(),
                version: None,
            };

            let resp1 = client
                .push(base_payload.clone(), None)
                .await
                .expect("initial push");

            let mut host_a_payload = base_payload.clone();
            host_a_payload.dotfiles = vec![make_dotfile("alpha", "host-a")];
            let resp2 = client
                .push(host_a_payload, Some(resp1.version))
                .await
                .expect("host a push");
            assert_eq!(resp2.version, resp1.version + 1);

            let mut host_b_payload = base_payload.clone();
            host_b_payload.dotfiles = vec![make_dotfile("alpha", "host-b")];
            let err = client
                .push(host_b_payload, Some(resp1.version))
                .await
                .expect_err("expected conflict");
            assert!(err.to_string().to_lowercase().contains("conflict"));

            let _ = shutdown_tx.send(());
            let _ = server.await;
        });
        std::env::remove_var("PRISM_SYNC_JWT_SECRET");
    });
}
