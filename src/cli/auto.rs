use std::fs;

use serde::{Deserialize, Serialize};

use super::{AutoArgs, CliContext};
use crate::context::{rules, ContextDetector};
use crate::error::PrismResult;

const AUTO_FILE: &str = "auto.json";

#[derive(Debug, Default, Serialize, Deserialize)]
struct AutoPreferences {
    default_theme: Option<String>,
}

pub fn handle_auto(args: AutoArgs, ctx: &CliContext) -> PrismResult<()> {
    if let Some(theme) = args.set {
        let mut prefs = load_preferences(ctx)?;
        prefs.default_theme = Some(theme.clone());
        save_preferences(ctx, &prefs)?;
        println!("Set default auto theme to '{theme}'.");
        return Ok(());
    }
    if args.clear {
        save_preferences(ctx, &AutoPreferences::default())?;
        println!("Cleared stored auto-theming preferences.");
        return Ok(());
    }

    let detector = ContextDetector::new(rules::load_rules()?, rules::load_manual_override()?);
    let snapshot = detector.detect(&std::env::current_dir()?)?;
    println!("Context snapshot: {}", snapshot.summary());
    if let Some(theme) = snapshot.suggested_theme {
        println!("Suggested theme: {theme}");
    } else {
        println!("No automatic theme match.");
    }
    if let Some(pref) = load_preferences(ctx)?.default_theme {
        println!("Preferred fallback theme: {pref}");
    }
    Ok(())
}

fn load_preferences(ctx: &CliContext) -> PrismResult<AutoPreferences> {
    let path = ctx.config_dir.join(AUTO_FILE);
    if path.exists() {
        let raw = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    } else {
        Ok(AutoPreferences::default())
    }
}

fn save_preferences(ctx: &CliContext, prefs: &AutoPreferences) -> PrismResult<()> {
    let path = ctx.config_dir.join(AUTO_FILE);
    fs::write(path, serde_json::to_string_pretty(prefs)?)?;
    Ok(())
}
