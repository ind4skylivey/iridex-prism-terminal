pub mod apply;
pub mod auto;
pub mod config;
pub mod daemon;
pub mod edit;
pub mod list;
pub mod preview;
pub mod revert;
pub mod sync;
pub mod widget;

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::core::apply::Shell;
use crate::core::loader;
use crate::core::theme::Theme;
use crate::error::PrismResult;
use crate::{ensure_config_dir, themes_root, user_themes_dir};

pub use apply::handle_apply;
pub use auto::handle_auto;
pub use config::handle_config;
pub use daemon::handle_daemon;
pub use edit::handle_edit;
pub use list::handle_list;
pub use preview::handle_preview;
pub use revert::handle_revert;
pub use sync::handle_sync;
pub use widget::handle_widget;

#[derive(Parser, Debug)]
#[command(
    name = "prism",
    about = "IRIDEX terminal aesthetic manager",
    version,
    propagate_version = true
)]
pub struct PrismCli {
    #[arg(long, global = true, action = clap::ArgAction::Count, help = "Increase log verbosity")]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Apply(ApplyArgs),
    Preview(PreviewArgs),
    List,
    Edit(EditArgs),
    Auto(AutoArgs),
    Widget(WidgetArgs),
    Sync(SyncArgs),
    Daemon(DaemonArgs),
    Config(ConfigArgs),
    Revert(RevertArgs),
}

#[derive(Args, Debug)]
pub struct ApplyArgs {
    pub theme: String,
    #[arg(long, value_enum)]
    pub shell: Option<ShellArg>,
}

#[derive(Args, Debug, Default)]
pub struct PreviewArgs {
    pub theme: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct EditArgs {
    pub theme: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct AutoArgs {
    #[arg(long, value_name = "THEME")]
    pub set: Option<String>,
    #[arg(long)]
    pub clear: bool,
}

#[derive(Args, Debug)]
pub struct WidgetArgs {
    #[command(subcommand)]
    pub command: WidgetCommands,
}

#[derive(Subcommand, Debug)]
pub enum WidgetCommands {
    Add {
        name: String,
    },
    Remove {
        name: String,
    },
    List,
    Configure {
        name: String,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        value: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct SyncArgs {
    #[command(subcommand)]
    pub command: SyncCommands,
}

#[derive(Subcommand, Debug)]
pub enum SyncCommands {
    Push,
    Pull,
    Status,
    Configure(SyncConfigureArgs),
    History,
    Rollback,
    Dotfiles(DotfileArgs),
    Jwt(JwtArgs),
    Serve(SyncServeArgs),
    Journal(SyncJournalArgs),
}

#[derive(Args, Debug)]
pub struct DaemonArgs {
    #[command(subcommand)]
    pub command: DaemonCommands,
}

#[derive(Subcommand, Debug)]
pub enum DaemonCommands {
    Start,
    Stop,
    Status,
    Enable,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Get { key: String },
    Set { key: String, value: String },
    Edit,
    Reset,
}

#[derive(Args, Debug)]
pub struct DotfileArgs {
    #[command(subcommand)]
    pub command: DotfileCommands,
}

#[derive(Subcommand, Debug)]
pub enum DotfileCommands {
    List,
    Restore {
        name: String,
        #[arg(long)]
        destination: Option<PathBuf>,
    },
    RestoreAll {
        #[arg(long)]
        destination: Option<PathBuf>,
    },
    Exclude {
        name: String,
    },
    Include {
        name: String,
    },
    Exclusions,
}

#[derive(Args, Debug)]
pub struct SyncServeArgs {
    #[arg(
        long,
        value_parser = clap::value_parser!(SocketAddr),
        default_value = "127.0.0.1:7878"
    )]
    pub listen: SocketAddr,
}

#[derive(Args, Debug, Default)]
pub struct SyncConfigureArgs {
    #[arg(long, value_name = "URL")]
    pub endpoint: Option<String>,
    #[arg(long)]
    pub clear_endpoint: bool,
    #[arg(long, value_name = "FILE")]
    pub ca_bundle: Option<PathBuf>,
    #[arg(long)]
    pub clear_ca_bundle: bool,
    #[arg(long)]
    pub insecure: bool,
    #[arg(long = "no-insecure")]
    pub no_insecure: bool,
    #[arg(long)]
    pub show: bool,
}

#[derive(Args, Debug)]
pub struct JwtArgs {
    #[command(subcommand)]
    pub command: JwtCommands,
}

#[derive(Subcommand, Debug)]
pub enum JwtCommands {
    Issue {
        #[arg(long, default_value = "local-dev")]
        subject: String,
        #[arg(long, default_value_t = 3600)]
        ttl: u64,
        #[arg(long)]
        secret: Option<String>,
        #[arg(long)]
        no_store: bool,
    },
}

#[derive(Args, Debug)]
pub struct SyncJournalArgs {
    #[command(subcommand)]
    pub command: SyncJournalCommands,
}

#[derive(Subcommand, Debug)]
pub enum SyncJournalCommands {
    List,
    Prune {
        #[arg(long, default_value_t = 20)]
        keep: usize,
    },
}

#[derive(Args, Debug)]
pub struct RevertArgs {
    #[arg(long, value_enum)]
    pub shell: Option<ShellArg>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ShellArg {
    Zsh,
    Bash,
    Fish,
}

impl From<ShellArg> for Shell {
    fn from(value: ShellArg) -> Self {
        match value {
            ShellArg::Zsh => Shell::Zsh,
            ShellArg::Bash => Shell::Bash,
            ShellArg::Fish => Shell::Fish,
        }
    }
}

pub struct CliContext {
    pub config_dir: PathBuf,
    pub built_in_themes: PathBuf,
    pub user_themes: PathBuf,
}

impl CliContext {
    pub fn new() -> PrismResult<Self> {
        Ok(Self {
            config_dir: ensure_config_dir()?,
            built_in_themes: themes_root()?,
            user_themes: user_themes_dir()?,
        })
    }

    pub fn load_theme(&self, name: &str) -> PrismResult<Theme> {
        loader::load_theme_by_name(name)
    }
}

pub fn run(cli: PrismCli) -> PrismResult<()> {
    if cli.verbose > 0 {
        log::set_max_level(log::LevelFilter::Debug);
    }

    let ctx = CliContext::new()?;
    match cli.command {
        Commands::Apply(args) => handle_apply(args, &ctx),
        Commands::Preview(args) => handle_preview(args, &ctx),
        Commands::List => handle_list(&ctx),
        Commands::Edit(args) => handle_edit(args, &ctx),
        Commands::Auto(args) => handle_auto(args, &ctx),
        Commands::Widget(args) => handle_widget(args, &ctx),
        Commands::Sync(args) => handle_sync(args, &ctx),
        Commands::Daemon(args) => handle_daemon(args, &ctx),
        Commands::Config(args) => handle_config(args, &ctx),
        Commands::Revert(args) => handle_revert(args, &ctx),
    }
}
