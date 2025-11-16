use super::CliContext;
use crate::core::loader::{self, ThemeSource};
use crate::error::PrismResult;

pub fn handle_list(_ctx: &CliContext) -> PrismResult<()> {
    let themes = loader::list_available()?;
    println!("Available themes ({} total):", themes.len());
    for entry in themes {
        let origin = match entry.source {
            ThemeSource::BuiltIn => "built-in",
            ThemeSource::User => "user",
            ThemeSource::Palette => "shared palette",
        };
        println!("- {} ({})", entry.theme.metadata.name, origin);
    }
    Ok(())
}
