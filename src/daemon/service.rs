use std::net::SocketAddr;

use crate::context::detector::ContextDetector;
use crate::context::rules;
use crate::core::loader;
use crate::core::theme::Theme;
use crate::daemon::ipc::IpcServer;
use crate::daemon::watcher::ContextWatcher;
use crate::ensure_config_dir;
use crate::error::PrismResult;
use crate::widgets::{self, WidgetManager};

pub struct PrismDaemon {
    addr: SocketAddr,
}

impl PrismDaemon {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn start(&self) -> PrismResult<()> {
        let detector = ContextDetector::new(rules::load_rules()?, rules::load_manual_override()?);
        let root = std::env::current_dir()?;
        let config_dir = ensure_config_dir()?;
        let enabled = widgets::storage::load_enabled(&config_dir)?;
        let widget_manager = WidgetManager::from_names(&enabled);
        let fallback_theme = resolve_fallback_theme()?;
        let mut watcher = ContextWatcher::new(detector, root, widget_manager, fallback_theme);
        let ipc = IpcServer::new(self.addr);

        tokio::select! {
            result = watcher.run() => result,
            result = ipc.run() => result,
        }
    }
}

fn resolve_fallback_theme() -> PrismResult<Theme> {
    if let Ok(theme) = loader::load_theme_by_name("cyberpunk") {
        return Ok(theme);
    }
    let mut entries = loader::list_available()?;
    let first = entries
        .pop()
        .ok_or_else(|| crate::error::PrismError::new("no themes available"))?;
    Theme::load(&first.path)
}
