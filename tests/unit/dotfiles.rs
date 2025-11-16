use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use tempfile::TempDir;

use prism::error::PrismResult;
use prism::sync::dotfiles;

fn with_config_dir<T>(test: impl FnOnce(&Path) -> T) -> T {
    let dir = TempDir::new().expect("config dir");
    env::set_var("PRISM_CONFIG_DIR", dir.path());
    let result = test(dir.path());
    env::remove_var("PRISM_CONFIG_DIR");
    result
}

#[test]
fn exclusions_round_trip() -> PrismResult<()> {
    with_config_dir(|_| {
        let list = vec!["zshrc".into(), "gitconfig".into(), "zshrc".into()];
        dotfiles::save_exclusions(&list)?;
        let loaded = dotfiles::load_exclusions()?;
        assert_eq!(loaded, vec!["gitconfig", "zshrc"]);
        Ok(())
    })
}

#[test]
fn restore_to_creates_backup_and_resets_content() -> PrismResult<()> {
    with_config_dir(|_| {
        let root = dotfiles::dotfiles_root()?;
        fs::create_dir_all(&root)?;
        let tracked = root.join("zshrc");
        fs::write(&tracked, "export PROMPT=1")?;

        let target_dir = TempDir::new().expect("dest dir");
        let target = target_dir.path().join("zshrc");
        let first = dotfiles::restore_to("zshrc", &target)?;
        assert!(first.overwritten_backup.is_none());
        assert!(first.snapshot_path.exists());
        assert_eq!(fs::read_to_string(&first.snapshot_path)?, "export PROMPT=1");
        assert_eq!(fs::read_to_string(&target)?, "export PROMPT=1");

        fs::write(&target, "local override")?;
        let outcome = dotfiles::restore_to("zshrc", &target)?;
        assert!(outcome.overwritten_backup.is_some());
        assert_eq!(fs::read_to_string(&target)?, "export PROMPT=1");
        let backup_path = outcome.overwritten_backup.unwrap();
        assert!(backup_path.exists());
        assert_eq!(fs::read_to_string(&backup_path)?, "local override");
        assert!(outcome.snapshot_path.exists());
        assert_eq!(fs::read_to_string(&outcome.snapshot_path)?, "export PROMPT=1");
        Ok(())
    })
}
