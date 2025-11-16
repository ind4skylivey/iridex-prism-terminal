use super::{CliContext, WidgetArgs, WidgetCommands};
use crate::error::PrismResult;
use crate::widgets::storage;

pub fn handle_widget(args: WidgetArgs, ctx: &CliContext) -> PrismResult<()> {
    let mut widgets = storage::load_enabled(&ctx.config_dir)?;
    match args.command {
        WidgetCommands::Add { name } => {
            if !widgets.contains(&name) {
                widgets.push(name.clone());
                widgets.sort();
                println!("Widget '{name}' enabled.");
                storage::save_enabled(&ctx.config_dir, &widgets)?;
            } else {
                println!("Widget '{name}' is already enabled.");
            }
        }
        WidgetCommands::Remove { name } => {
            let initial_len = widgets.len();
            widgets.retain(|w| w != &name);
            if widgets.len() != initial_len {
                storage::save_enabled(&ctx.config_dir, &widgets)?;
                println!("Widget '{name}' removed.");
            } else {
                println!("Widget '{name}' was not enabled.");
            }
        }
        WidgetCommands::List => {
            if widgets.is_empty() {
                println!("No widgets enabled yet.");
            } else {
                widgets.sort();
                println!("Enabled widgets:");
                for widget in &widgets {
                    println!("- {widget}");
                }
            }
        }
        WidgetCommands::Configure { name, key, value } => {
            configure_widget(ctx, &name, key, value)?;
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
    let mut config = storage::load_settings(&ctx.config_dir)?;
    match (key, value) {
        (Some(key), Some(value)) => {
            let entry = config
                .entry(name.to_string())
                .or_insert_with(Default::default);
            entry.insert(key.clone(), value.clone());
            println!("Set {name}.{key} = {value}");
            storage::save_settings(&ctx.config_dir, &config)?;
        }
        (Some(key), None) => {
            let mut remove_entry = false;
            if let Some(entry) = config.get_mut(name) {
                if entry.remove(&key).is_some() {
                    println!("Removed {name}.{key}");
                } else {
                    println!("{name}.{key} was not set");
                }
                remove_entry = entry.is_empty();
            } else {
                println!("No configuration stored for '{name}'.");
            }
            if remove_entry {
                config.remove(name);
            }
            storage::save_settings(&ctx.config_dir, &config)?;
        }
        _ => {
            if let Some(entry) = config.get(name) {
                if entry.is_empty() {
                    println!("No configuration stored for '{name}'.");
                } else {
                    println!("Settings for {name}:");
                    for (key, value) in entry {
                        println!("- {key} = {value}");
                    }
                }
            } else {
                println!("No configuration stored for '{name}'.");
            }
        }
    }
    Ok(())
}
