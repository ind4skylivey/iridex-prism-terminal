use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::catalog::ThemeCatalog;
use crate::cli::apply::apply_theme;
use crate::error::PrismResult;
use crate::themes::ThemeId;

pub fn handle_list(catalog: &ThemeCatalog) -> PrismResult<()> {
    let mut items = Vec::new();
    let mut themes = Vec::new();

    for entry in catalog.iter() {
        if entry.theme.meta.id == ThemeId::ThemeTemplate {
            continue;
        }

        let name = &entry.theme.meta.name;

        // Create color preview swatches from the first 5 base colors
        let mut swatches = String::new();
        for color in entry.theme.palette.base.iter().take(5) {
            if let Some((r, g, b)) = hex_to_rgb(color) {
                swatches.push_str(&format!("{}", "██".truecolor(r, g, b)));
            }
        }

        let desc = if entry.theme.meta.description.len() > 40 {
            format!("{}...", &entry.theme.meta.description[..37])
        } else {
            entry.theme.meta.description.clone()
        };

        // Format: "ThemeName  [color swatches]  Description"
        items.push(format!(
            "{:<18} {} {}",
            name.bold(),
            swatches,
            desc.dimmed().italic()
        ));
        themes.push(&entry.theme);
    }

    if items.is_empty() {
        println!("No themes found.");
        return Ok(());
    }

    println!(
        "\n{} Navigate with ↑/↓ arrows. Press {} to apply, {} to cancel.\n",
        "ℹ".cyan().bold(),
        "Enter".green().bold(),
        "Esc".yellow().bold()
    );

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a theme")
        .default(0)
        .items(&items)
        .interact_opt()
        .map_err(|e| crate::error::PrismError::new(format!("Selection error: {}", e)))?;

    if let Some(index) = selection {
        let selected_theme = themes[index];
        apply_theme(selected_theme)?;

        println!(
            "\n{} Applied theme: {}",
            "✔".green().bold(),
            selected_theme.meta.name.cyan().bold()
        );
        println!(
            "{} Run {} to see changes in a new shell session.",
            "ℹ".blue(),
            "exec $SHELL".yellow()
        );
    } else {
        println!("\n{} Selection cancelled.", "✖".yellow());
    }

    Ok(())
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}
