use std::fs;

use super::{CliContext, WidgetArgs, WidgetCommands};
use crate::error::PrismResult;

const WIDGET_FILE: &str = "widgets.json";

pub fn handle_widget(args: WidgetArgs, ctx: &CliContext) -> PrismResult<()> {
    let mut widgets = read_widgets(ctx)?;
    match args.command {
        WidgetCommands::Add { name } => {
            if !widgets.contains(&name) {
                widgets.push(name.clone());
                println!("Widget '{name}' enabled.");
            }
            write_widgets(ctx, &widgets)?;
        }
        WidgetCommands::Remove { name } => {
            widgets.retain(|w| w != &name);
            write_widgets(ctx, &widgets)?;
            println!("Widget '{name}' removed.");
        }
        WidgetCommands::List => {
            if widgets.is_empty() {
                println!("No widgets enabled yet.");
            } else {
                println!("Enabled widgets:");
                for widget in &widgets {
                    println!("- {widget}");
                }
            }
        }
        WidgetCommands::Configure { name } => {
            println!("Configuration UI for '{name}' is not implemented yet.");
        }
    }
    Ok(())
}

fn read_widgets(ctx: &CliContext) -> PrismResult<Vec<String>> {
    let path = ctx.config_dir.join(WIDGET_FILE);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(vec![])
    }
}

fn write_widgets(ctx: &CliContext, widgets: &[String]) -> PrismResult<()> {
    let path = ctx.config_dir.join(WIDGET_FILE);
    fs::write(path, serde_json::to_string_pretty(widgets)?)?;
    Ok(())
}
