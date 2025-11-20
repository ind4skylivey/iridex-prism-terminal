use clap::Parser;

use prism::catalog::load_catalog;
use prism::cli::{PrismCli, PrismCommand};
use prism::themes::ThemeId;

#[test]
fn parses_apply_command() {
    let cli = PrismCli::try_parse_from(["prism", "apply", "lavender-core"])
        .expect("cli parses apply command");
    match cli.command {
        PrismCommand::Apply(args) => assert_eq!(args.theme, "lavender-core"),
        other => panic!("expected apply command, got {other:?}"),
    }
}

#[test]
fn parses_list_command() {
    let cli = PrismCli::try_parse_from(["prism", "list"]).expect("cli parses list command");
    assert!(matches!(cli.command, PrismCommand::List));
}

#[test]
fn load_catalog_exposes_custom_theme() {
    let catalog = load_catalog().expect("catalog loads");
    assert!(catalog.get(ThemeId::Custom).is_some());
}

#[test]
fn catalog_resolves_theme_by_name() {
    let catalog = load_catalog().expect("catalog loads");
    let entry = catalog
        .resolve("Lavender Core")
        .expect("lavender-core theme exists");
    assert_eq!(entry.theme.meta.slug, "lavender-core");
}
