use prism::core::loader;
use prism::error::PrismResult;

use super::helpers::TestEnv;

const MINIMAL_THEME: &str = r##"
[metadata]
name = "Nebula"
author = "Infra"

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
fn load_theme_by_name_reads_from_theme_root() -> PrismResult<()> {
    let env = TestEnv::new();
    env.write_builtin_theme("Nebula", MINIMAL_THEME);

    let theme = loader::load_theme_by_name("nebula")?;

    assert_eq!(theme.metadata.name, "Nebula");
    Ok(())
}

#[test]
fn load_theme_by_name_reads_from_user_themes() -> PrismResult<()> {
    let env = TestEnv::new();
    env.write_user_theme("Nebula", MINIMAL_THEME);

    let theme = loader::load_theme_by_name("nebula")?;

    assert_eq!(theme.metadata.name, "Nebula");
    assert_eq!(theme.metadata.author, "Infra");
    Ok(())
}

#[test]
fn load_theme_by_name_errors_for_missing_theme() {
    let _env = TestEnv::new();
    let err = loader::load_theme_by_name("ghost").expect_err("missing theme should error");
    assert!(
        err.to_string().contains("not found"),
        "unexpected error: {err:?}"
    );
}
