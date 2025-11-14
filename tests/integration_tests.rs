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
    let lock = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock();
    let _guard = lock.expect("lock poisoned");
    let dir = tempfile::tempdir().expect("config dir");
    std::env::set_var("PRISM_CONFIG_DIR", dir.path());
    std::env::set_var("PRISM_DISABLE_SHELL_HOOKS", "1");
    test();
    std::env::remove_var("PRISM_CONFIG_DIR");
    std::env::remove_var("PRISM_DISABLE_SHELL_HOOKS");
}

#[test]
fn load_builtin_themes() {
    with_isolated_env(|| {
        let themes = loader::list_available().expect("themes");
        assert!(!themes.is_empty());
        for entry in themes {
            let theme = Theme::load(&entry.path).expect("load theme");
            theme.validate().expect("validate");
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
                Some(endpoint),
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
            };

            client.push(payload.clone()).await.expect("push");
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
