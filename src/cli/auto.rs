use super::{AutoArgs, CliContext};
use crate::context::rules;
use crate::context::ContextDetector;
use crate::error::PrismResult;

pub fn handle_auto(args: AutoArgs, _ctx: &CliContext) -> PrismResult<()> {
    if let Some(theme) = args.set {
        rules::write_manual_override(Some(&theme))?;
        println!("Manual override set to '{theme}'.");
        return Ok(());
    }
    if args.clear {
        rules::write_manual_override(None)?;
        println!("Manual override cleared.");
        return Ok(());
    }

    let rule_config = rules::load_rules()?;
    let manual = rules::load_manual_override()?;
    let detector = ContextDetector::new(rule_config, manual.clone());
    let snapshot = detector.detect(&std::env::current_dir()?)?;
    println!("Context snapshot: {}", snapshot.summary());
    if let Some(theme) = snapshot.suggested_theme {
        println!("Suggested theme: {theme}");
        if manual.is_some() {
            println!("(Manual override active. Use `prism auto --clear` to disable.)");
        }
    } else {
        println!("No automatic theme match.");
    }
    Ok(())
}
