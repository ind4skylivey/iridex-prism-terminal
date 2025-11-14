use super::{CliContext, EditArgs};
use crate::error::PrismResult;
use std::path::PathBuf;

use crate::tui::editor::edit_theme;

pub fn handle_edit(args: EditArgs, ctx: &CliContext) -> PrismResult<()> {
    let theme_name = args.theme.unwrap_or_else(|| "cyberpunk".into());
    let theme = ctx.load_theme(&theme_name)?;
    let destination = theme_destination(ctx, &theme_name);
    edit_theme(theme, &destination).or_else(|err| {
        println!("Editor placeholder: {err}");
        Ok(())
    })
}

fn theme_destination(ctx: &CliContext, name: &str) -> PathBuf {
    ctx.user_themes.join(format!("{name}.toml"))
}
