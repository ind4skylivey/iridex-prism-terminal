use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use crate::catalog::{load_catalog, CatalogEntry, ThemeCatalog};
use crate::error::{PrismError, PrismResult};
use crate::themes::ThemeId;
use crate::{ensure_config_dir, user_themes_dir};

pub mod init;
pub mod apply;
pub mod dev;
pub mod info;
pub mod list;

#[derive(Parser, Debug)]
#[command(
    name = "prism",
    version,
    about = "IRIDEX-inspired terminal aesthetic engine"
)]
pub struct PrismCli {
    #[arg(long, short, global = true, action = clap::ArgAction::Count, help = "Increase log verbosity")]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: PrismCommand,
}

#[derive(Subcommand, Debug)]
pub enum PrismCommand {
    /// List all built-in themes with their short descriptions
    List,
    /// Print the shell scripts used to activate a theme
    Apply(ApplyArgs),
    /// Display palette previews; optionally focus on a single theme
    Preview(PreviewArgs),
    /// Show developer metadata and file paths for a theme
    Dev(DevArgs),
    /// Prepare ~/.config/prism and ensure the Custom theme placeholder exists
    Init(InitArgs),
    /// Show detailed information about a specific theme
    Info(DevArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(long, help = "Undo the initialization (remove Prism from shell config)")]
    pub undo: bool,

    #[arg(long, help = "Purge the configuration directory (only with --undo)")]
    pub purge: bool,

    #[arg(long, help = "Immediately apply a theme")]
    pub theme: Option<String>,
}

#[derive(Args, Debug)]
pub struct ApplyArgs {
    #[arg(value_name = "THEME", help = "Theme slug or display name")]
    pub theme: String,
}

#[derive(Args, Debug, Default)]
pub struct PreviewArgs {
    #[arg(value_name = "THEME", help = "Optional theme slug to preview")]
    pub theme: Option<String>,
}

#[derive(Args, Debug)]
pub struct DevArgs {
    #[arg(value_name = "THEME", help = "Theme slug or display name")]
    pub theme: String,
}

pub fn run(cli: PrismCli) -> PrismResult<()> {
    if cli.verbose > 0 {
        log::set_max_level(log::LevelFilter::Debug);
    }

    match cli.command {
        PrismCommand::Init(args) => init::handle_init(args),
        PrismCommand::List => {
            let catalog = load_catalog()?;
            list::handle_list(&catalog)
        }
        PrismCommand::Info(args) => {
            let catalog = load_catalog()?;
            info::handle_info(&catalog, args)
        }
        PrismCommand::Apply(args) => {
            let catalog = load_catalog()?;
            handle_apply(&catalog, args)
        }
        PrismCommand::Preview(args) => {
            let catalog = load_catalog()?;
            handle_preview(&catalog, args)
        }
        PrismCommand::Dev(args) => {
            dev::handle_dev(args)
        }
    }
}



pub fn handle_apply(catalog: &ThemeCatalog, args: ApplyArgs) -> PrismResult<()> {
    let entry = resolve_theme(catalog, &args.theme)?;
    apply::apply_theme(&entry.theme)?;
    
    println!(
        "Applied {} ({}):",
        entry.theme.meta.name, entry.theme.meta.slug
    );
    println!("Source the matching script in your shell rc file to activate the theme.");
    Ok(())
}

fn handle_preview(catalog: &ThemeCatalog, args: PreviewArgs) -> PrismResult<()> {
    if let Some(theme) = args.theme {
        let entry = resolve_theme(catalog, &theme)?;
        preview_entry(entry);
    } else {
        for entry in catalog.iter() {
            preview_entry(entry);
            println!();
        }
    }
    Ok(())
}





fn resolve_theme<'catalog>(
    catalog: &'catalog ThemeCatalog,
    query: &str,
) -> PrismResult<&'catalog CatalogEntry> {
    catalog.resolve(query).ok_or_else(|| {
        PrismError::new(format!(
            "unknown theme '{query}'. Run `prism list` to see available slugs."
        ))
    })
}

fn preview_entry(entry: &CatalogEntry) {
    println!("{} ({})", entry.theme.meta.name, entry.theme.meta.slug);
    print!("  Palette: ");
    for color in entry.theme.palette.base.iter().take(8) {
        if let Some((r, g, b)) = hex_to_rgb(color) {
            print!("{}", "██".truecolor(r, g, b));
        } else {
            print!(" {} ", color);
        }
    }
    println!();
    if !entry.theme.meta.description.is_empty() {
        println!("  {}", entry.theme.meta.description);
    }
}

fn hex_to_rgb(value: &str) -> Option<(u8, u8, u8)> {
    let normalized = value.trim().trim_start_matches('#');
    if normalized.len() < 6 {
        return None;
    }
    let (r, g, b) = (
        u8::from_str_radix(&normalized[0..2], 16).ok()?,
        u8::from_str_radix(&normalized[2..4], 16).ok()?,
        u8::from_str_radix(&normalized[4..6], 16).ok()?,
    );
    Some((r, g, b))
}
