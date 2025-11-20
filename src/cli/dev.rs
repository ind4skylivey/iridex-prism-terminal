use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::catalog::load_catalog;
use crate::cli::apply::apply_theme;
use crate::cli::DevArgs;
use crate::error::{PrismError, PrismResult};
use crate::themes::ThemeId;

pub fn handle_dev(args: DevArgs) -> PrismResult<()> {
    // 1. Initial load to find the file path
    let catalog = load_catalog()?;
    let entry = catalog
        .resolve(&args.theme)
        .ok_or_else(|| PrismError::new(format!("Theme '{}' not found", args.theme)))?;

    let theme_path = entry.palette_path.clone();
    let theme_id = entry.theme.meta.id;

    println!("Δ Prism Dev Mode");
    println!("  Watching: {}", theme_path.display());
    println!(
        "  Theme:    {} ({})",
        entry.theme.meta.name, entry.theme.meta.slug
    );
    println!("  Press Ctrl+C to exit.");

    // 2. Setup Watcher
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| PrismError::new(format!("Failed to create watcher: {}", e)))?;

    watcher
        .watch(&theme_path, RecursiveMode::NonRecursive)
        .map_err(|e| PrismError::new(format!("Failed to watch file: {}", e)))?;

    // 3. Event Loop
    loop {
        match rx.recv() {
            Ok(res) => {
                match res {
                    Ok(event) => {
                        // We only care about Write or Create (if re-created)
                        // notify 6.0 events are a bit generic, usually Modify(Data)
                        if event.kind.is_modify() || event.kind.is_create() {
                            // Debounce slightly by sleeping?
                            // Better: just try to reload.
                            // If editors save atomically (rename), we might get Create.

                            // Give FS a moment to settle
                            std::thread::sleep(Duration::from_millis(100));

                            match reload_and_apply(theme_id, &theme_path) {
                                Ok(name) => println!("  Δ Reloaded: {}", name),
                                Err(e) => eprintln!("  ! Error reloading: {}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("  ! Watch error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("  ! Channel error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn reload_and_apply(id: ThemeId, _path: &Path) -> PrismResult<String> {
    // We reload the whole catalog to ensure we get the fresh JSON content
    // Optimization: We could just parse the specific file if we refactored catalog loading,
    // but loading 15 JSONs is fast enough for dev mode.
    let catalog = load_catalog()?;

    // Find the theme again by ID (slug might have changed? unlikely for dev mode on same file)
    // But if we are editing "Custom", ID is Custom.
    let entry = catalog
        .get(id)
        .ok_or_else(|| PrismError::new("Theme disappeared from catalog (parsing error?)"))?;

    apply_theme(&entry.theme)?;

    Ok(entry.theme.meta.name.clone())
}
