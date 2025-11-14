use super::{ApplyArgs, CliContext, ShellArg};
use crate::error::PrismResult;

pub fn handle_apply(args: ApplyArgs, ctx: &CliContext) -> PrismResult<()> {
    let shell = args.shell.unwrap_or(ShellArg::Zsh).into();
    let theme = ctx.load_theme(&args.theme)?;
    theme.apply(shell)?;
    println!(
        "Applied theme '{}' for {:?}. Source the generated script from {}",
        theme.metadata.name,
        shell,
        ctx.config_dir.display()
    );
    Ok(())
}
