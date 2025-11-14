use super::{CliContext, WidgetArgs, WidgetCommands};
use crate::error::{PrismError, PrismResult};
use crate::widgets::storage::{self, WidgetSettings};

pub fn handle_widget(args: WidgetArgs, ctx: &CliContext) -> PrismResult<()> {
    match args.command {
        WidgetCommands::Add { name } => add_widget(ctx, &name),
        WidgetCommands::Remove { name } => remove_widget(ctx, &name),
        WidgetCommands::List => list_widgets(ctx),
        WidgetCommands::Configure { name, key, value } => configure_widget(ctx, &name, key, value),
    }
}

fn add_widget(ctx: &CliContext, name: &str) -> PrismResult<()> {
    let mut widgets = storage::load_enabled(&ctx.config_dir)?;
    if !widgets.iter().any(|w| w == name) {
        widgets.push(name.to_string());
        widgets.sort();
        println!("Widget '{name}' enabled.");
    }
    storage::save_enabled(&ctx.config_dir, &widgets)
}

fn remove_widget(ctx: &CliContext, name: &str) -> PrismResult<()> {
    let mut widgets = storage::load_enabled(&ctx.config_dir)?;
    let original_len = widgets.len();
    widgets.retain(|w| w != name);
    if widgets.len() < original_len {
        println!("Widget '{name}' removed.");
    }
    storage::save_enabled(&ctx.config_dir, &widgets)
}

fn list_widgets(ctx: &CliContext) -> PrismResult<()> {
    let widgets = storage::load_enabled(&ctx.config_dir)?;
    if widgets.is_empty() {
        println!("No widgets enabled yet.");
    } else {
        println!("Enabled widgets:");
        for widget in widgets {
            println!("- {widget}");
        }
    }
    Ok(())
}

fn configure_widget(
    ctx: &CliContext,
    name: &str,
    key: Option<String>,
    value: Option<String>,
) -> PrismResult<()> {
    match (key, value) {
        (Some(k), Some(v)) => {
            storage::upsert_setting(&ctx.config_dir, name, &k, &v)?;
            println!("Set {name}.{k} = {v}");
            Ok(())
        }
        (Some(_), None) | (None, Some(_)) => Err(PrismError::new(
            "Provide both --key and --value to configure a widget setting.",
        )),
        (None, None) => {
            let settings = storage::load_settings(&ctx.config_dir)?;
            show_widget_settings(name, &settings);
            Ok(())
        }
    }
}

fn show_widget_settings(name: &str, settings: &WidgetSettings) {
    match settings.get(name) {
        Some(entries) if !entries.is_empty() => {
            println!("Settings for '{name}':");
            for (key, value) in entries {
                println!("- {key} = {value}");
            }
        }
        _ => println!("No settings stored for '{name}'."),
    }
}
