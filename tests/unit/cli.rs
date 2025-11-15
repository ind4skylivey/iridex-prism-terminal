use clap::Parser;

use prism::cli::{CliContext, Commands, PrismCli, ShellArg};
use prism::error::PrismResult;

use super::helpers::TestEnv;

const SAMPLE_THEME: &str = r##"
[metadata]
name = "Solstice"
author = "QA"

[colors]
background = "#000000"
foreground = "#ffffff"
black = "#000000"
red = "#ff5555"
green = "#50fa7b"
yellow = "#f1fa8c"
blue = "#bd93f9"
magenta = "#ff79c6"
cyan = "#8be9fd"
white = "#bbbbbb"

[colors.bright]
black = "#4d4d4d"
red = "#ff6e67"
green = "#5af78e"
yellow = "#f4f99d"
blue = "#caa9fa"
magenta = "#ff92d0"
cyan = "#9aedfe"
white = "#f8f8f2"
"##;

#[test]
fn parses_apply_command_with_shell_and_verbose_flags() {
    let cli = PrismCli::try_parse_from([
        "prism",
        "--verbose",
        "--verbose",
        "apply",
        "solstice",
        "--shell",
        "zsh",
    ])
    .expect("cli parsing");

    assert_eq!(cli.verbose, 2);
    match cli.command {
        Commands::Apply(args) => {
            assert_eq!(args.theme, "solstice");
            assert!(matches!(args.shell, Some(ShellArg::Zsh)));
        }
        other => panic!("expected apply command, got {other:?}"),
    }
}

#[test]
fn cli_context_loads_theme_using_temp_env() -> PrismResult<()> {
    let env = TestEnv::new();
    env.write_builtin_theme("Solstice", SAMPLE_THEME);

    let ctx = CliContext::new()?;
    let theme = ctx.load_theme("solstice")?;

    assert_eq!(theme.metadata.name, "Solstice");
    assert_eq!(theme.metadata.author, "QA");
    Ok(())
}
