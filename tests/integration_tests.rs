use std::path::Path;

use prism::context::ContextDetector;
use prism::core::apply::Shell;
use prism::core::loader;
use prism::core::theme::Theme;

#[test]
fn load_builtin_themes() {
    let themes = loader::list_available().expect("themes");
    assert!(!themes.is_empty());
    for entry in themes {
        let theme = Theme::load(&entry.path).expect("load theme");
        theme.validate().expect("validate");
    }
}

#[test]
fn context_detector_runs() {
    let detector = ContextDetector::new(None);
    let snapshot = detector.detect(Path::new("."));
    assert!(snapshot.is_ok());
}

#[test]
fn apply_is_dry_run_ready() {
    let themes = loader::list_available().expect("themes");
    let theme = Theme::load(&themes[0].path).expect("theme");
    let dir = tempfile::tempdir().expect("tmp");
    prism::core::apply::apply_theme(&theme, Shell::Zsh, dir.path()).expect("apply");
}
