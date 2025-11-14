use std::net::SocketAddr;

use crate::context::{rules, ContextDetector};
use crate::daemon::ipc::IpcServer;
use crate::daemon::watcher::ContextWatcher;
use crate::error::PrismResult;

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
        let mut watcher = ContextWatcher::new(detector, root);
        let ipc = IpcServer::new(self.addr);

        tokio::select! {
            result = watcher.run() => result,
            result = ipc.run() => result,
        }
    }
}
