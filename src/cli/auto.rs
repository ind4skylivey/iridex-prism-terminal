use super::CliContext;
use crate::context::ContextDetector;
use crate::core::theme::ContextRules;
use crate::error::PrismResult;

pub fn handle_auto(_ctx: &CliContext) -> PrismResult<()> {
    let detector = ContextDetector::new(Some(ContextRules::default()));
    let snapshot = detector.detect(&std::env::current_dir()?)?;
    println!("Context snapshot: {}", snapshot.summary());
    if let Some(theme) = snapshot.suggested_theme {
        println!("Suggested theme: {theme}");
    } else {
        println!("No automatic theme match.");
    }
    Ok(())
}
