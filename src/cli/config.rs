use std::collections::BTreeMap;
use std::fs;

use super::{CliContext, ConfigArgs, ConfigCommands};
use crate::error::PrismResult;

const CONFIG_FILE: &str = "config.json";

pub fn handle_config(args: ConfigArgs, ctx: &CliContext) -> PrismResult<()> {
    let mut config = read_config(ctx)?;
    match args.command {
        ConfigCommands::Get { key } => {
            if let Some(value) = config.get(&key) {
                println!("{key} = {value}");
            } else {
                println!("{key} is not set");
            }
        }
        ConfigCommands::Set { key, value } => {
            config.insert(key.clone(), value.clone());
            write_config(ctx, &config)?;
            println!("Updated {key}");
        }
        ConfigCommands::Edit => {
            println!(
                "Open {} in your favorite editor",
                ctx.config_dir.join(CONFIG_FILE).display()
            );
        }
        ConfigCommands::Reset => {
            config.clear();
            write_config(ctx, &config)?;
            println!("Configuration reset");
        }
    }
    Ok(())
}

fn read_config(ctx: &CliContext) -> PrismResult<BTreeMap<String, String>> {
    let path = ctx.config_dir.join(CONFIG_FILE);
    if path.exists() {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    } else {
        Ok(BTreeMap::new())
    }
}

fn write_config(ctx: &CliContext, config: &BTreeMap<String, String>) -> PrismResult<()> {
    let path = ctx.config_dir.join(CONFIG_FILE);
    fs::write(path, serde_json::to_string_pretty(config)?)?;
    Ok(())
}
