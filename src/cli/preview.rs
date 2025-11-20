use super::{CliContext, PreviewArgs};
use crate::core::loader;
use crate::error::{PrismError, PrismResult};
use crate::tui::preview::run_preview;

pub fn handle_preview(args: PreviewArgs, _ctx: &CliContext) -> PrismResult<()> {
    let entries = loader::list_available()?;
    let filtered = if let Some(name) = args.theme {
        let target = name.to_lowercase();
        let entry = entries
            .into_iter()
            .find(|entry| entry.theme.metadata.name.to_lowercase() == target)
            .ok_or_else(|| PrismError::new(format!("theme '{name}' not found")))?;
        vec![entry]
    } else {
        entries
    };
    run_preview(filtered)
}
