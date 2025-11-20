use super::{CliContext, RevertArgs, ShellArg};
use crate::core::apply::{self, Shell};
use crate::error::PrismResult;

pub fn handle_revert(args: RevertArgs, ctx: &CliContext) -> PrismResult<()> {
    let shell: Shell = args.shell.unwrap_or(ShellArg::Zsh).into();
    apply::revert(shell, &ctx.config_dir)?;
    println!("Reverted {shell:?} integration");
    Ok(())
}
