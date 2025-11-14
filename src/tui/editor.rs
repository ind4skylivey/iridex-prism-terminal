use crate::core::theme::Theme;
use crate::error::{PrismError, PrismResult};

pub fn edit_theme(_theme: &Theme) -> PrismResult<()> {
    Err(PrismError::new("TUI editor not implemented yet"))
}
