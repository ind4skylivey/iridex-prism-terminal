use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::{CliContext, DaemonArgs, DaemonCommands};
use crate::daemon::PrismDaemon;
use crate::error::PrismResult;

pub fn handle_daemon(args: DaemonArgs, _ctx: &CliContext) -> PrismResult<()> {
    match args.command {
        DaemonCommands::Start => start_daemon(),
        DaemonCommands::Stop => {
            println!("Send stop signal via IPC (not implemented)");
            Ok(())
        }
        DaemonCommands::Status => {
            println!("Daemon status probing is not implemented yet");
            Ok(())
        }
        DaemonCommands::Enable => {
            println!("System service integration placeholder");
            Ok(())
        }
    }
}

fn start_daemon() -> PrismResult<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9393);
    let daemon = PrismDaemon::new(addr);
    tokio::runtime::Runtime::new()?.block_on(async move { daemon.start().await })
}
