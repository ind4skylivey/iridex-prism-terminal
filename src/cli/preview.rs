use super::{CliContext, PreviewArgs};
use crate::core::loader;
use crate::core::theme::Theme;
use crate::error::PrismResult;
use crate::tui::preview::run_preview;

pub fn handle_preview(args: PreviewArgs, _ctx: &CliContext) -> PrismResult<()> {
    let themes: Vec<Theme> = if let Some(name) = args.theme {
        vec![loader::load_theme_by_name(&name)?]
    } else {
        loader::list_available()?
            .into_iter()
            .map(|entry| Theme::load(&entry.path))
            .collect::<PrismResult<_>>()?
    };
    run_preview(themes)
}
