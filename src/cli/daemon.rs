use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::process::Command;

use dirs::home_dir;

use super::{CliContext, DaemonArgs, DaemonCommands};
use crate::daemon::PrismDaemon;
use crate::error::{PrismError, PrismResult};

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
        DaemonCommands::Enable => enable_systemd_unit(),
    }
}

fn start_daemon() -> PrismResult<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9393);
    let daemon = PrismDaemon::new(addr);
    tokio::runtime::Runtime::new()?.block_on(async move { daemon.start().await })
}

fn enable_systemd_unit() -> PrismResult<()> {
    let home = home_dir().ok_or_else(|| PrismError::new("unable to locate home directory"))?;
    let unit_dir = home.join(".config/systemd/user");
    fs::create_dir_all(&unit_dir)?;
    let service_path = unit_dir.join("prism-daemon.service");
    let exe = std::env::current_exe()?;
    fs::write(&service_path, systemd_unit(&exe))?;
    println!("Wrote {}", service_path.display());
    match try_systemctl(&["--user", "daemon-reload"]) {
        Ok(_) => println!("Reloaded user systemd units."),
        Err(err) => {
            println!(
                "Could not reload units automatically ({err}). Run `systemctl --user daemon-reload` manually."
            );
        }
    }
    match try_systemctl(&["--user", "enable", "--now", "prism-daemon.service"]) {
        Ok(_) => {
            println!("Enabled and started prism-daemon.service via systemctl.");
            println!("Verify with: systemctl --user status prism-daemon.service");
        }
        Err(err) => {
            println!(
                "Automatic enable/start failed ({err}). Run `systemctl --user enable --now prism-daemon.service`."
            );
        }
    }
    Ok(())
}

fn systemd_unit(exe: &Path) -> String {
    let binary = exe.display();
    format!(
        "[Unit]\nDescription=PRISM Auto Theme Daemon\nAfter=network-online.target\n\n[Service]\nExecStart=\"{}\" daemon start\nRestart=on-failure\nEnvironment=RUST_LOG=info\n\n[Install]\nWantedBy=default.target\n",
        binary
    )
}

fn try_systemctl(args: &[&str]) -> PrismResult<()> {
    let status = Command::new("systemctl").args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(PrismError::new(format!(
            "systemctl {:?} exited with status {}",
            args, status
        )))
    }
}
