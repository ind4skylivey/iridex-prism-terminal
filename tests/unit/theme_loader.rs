use prism::core::loader;
use prism::error::PrismResult;
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

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
#[serial(theme_loader)]
fn load_theme_by_name_reads_from_theme_root() -> PrismResult<()> {
    let roots = ThemeTestRoots::new();
    roots.write_builtin_theme("Nebula", MINIMAL_THEME);

    let theme =
        loader::load_theme_by_name_with_roots("nebula", roots.builtin_root(), roots.user_root())?;

    assert_eq!(theme.metadata.name, "Nebula");
    Ok(())
}

#[test]
#[serial(theme_loader)]
fn load_theme_by_name_reads_from_user_themes() -> PrismResult<()> {
    let roots = ThemeTestRoots::new();
    roots.write_user_theme("Nebula", MINIMAL_THEME);

    let theme =
        loader::load_theme_by_name_with_roots("nebula", roots.builtin_root(), roots.user_root())?;

    assert_eq!(theme.metadata.name, "Nebula");
    assert_eq!(theme.metadata.author, "Infra");
    Ok(())
}

#[test]
#[serial(theme_loader)]
fn load_theme_by_name_errors_for_missing_theme() {
    let roots = ThemeTestRoots::new();
    let err =
        loader::load_theme_by_name_with_roots("ghost", roots.builtin_root(), roots.user_root())
            .expect_err("missing theme should error");
    assert!(
        err.to_string().contains("not found"),
        "unexpected error: {err:?}"
    );
}

struct ThemeTestRoots {
    builtin: TempDir,
    user: TempDir,
}

impl ThemeTestRoots {
    fn new() -> Self {
        Self {
            builtin: TempDir::new().expect("builtin dir"),
            user: TempDir::new().expect("user dir"),
        }
    }

    fn builtin_root(&self) -> &Path {
        self.builtin.path()
    }

    fn user_root(&self) -> &Path {
        self.user.path()
    }

    fn write_builtin_theme(&self, name: &str, body: &str) {
        self.write_theme(self.builtin_root(), name, body);
    }

    fn write_user_theme(&self, name: &str, body: &str) {
        self.write_theme(self.user_root(), name, body);
    }

    fn write_theme(&self, dir: &Path, name: &str, body: &str) {
        let path = dir.join(format!("{name}.toml"));
        fs::write(path, body).expect("theme file");
    }
}
