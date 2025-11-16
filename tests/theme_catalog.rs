use std::{collections::HashSet, path::Path};

use prism::core::{loader, shared_palette};
use prism::error::PrismResult;

fn shared_palettes() -> PrismResult<Vec<shared_palette::SharedPaletteEntry>> {
    shared_palette::load_shared_palettes(Path::new("themes/shared-palettes"))
}

#[test]
fn palette_artifacts_present_for_all_themes() -> PrismResult<()> {
    let palettes = shared_palettes()?;
    for palette in &palettes {
        let slug = palette.slug();
        let zsh = Path::new("themes")
            .join("zsh")
            .join(format!("{slug}.zsh-theme"));
        assert!(
            zsh.exists(),
            "missing zsh prompt for palette {}: {}",
            slug,
            zsh.display()
        );
        let fish = Path::new("themes")
            .join("fish")
            .join(format!("{slug}.fish"));
        assert!(
            fish.exists(),
            "missing fish prompt for palette {}: {}",
            slug,
            fish.display()
        );
        let bash = Path::new("themes").join("bash").join(format!("{slug}.sh"));
        assert!(
            bash.exists(),
            "missing bash prompt for palette {}: {}",
            slug,
            bash.display()
        );
        let doc = Path::new("docs").join(format!("{slug}.md"));
        assert!(
            doc.exists(),
            "missing doc for palette {}: {}",
            slug,
            doc.display()
        );
    }
    Ok(())
}

#[test]
fn loader_exposes_every_palette_entry() -> PrismResult<()> {
    let palettes = shared_palettes()?;
    let expected_names: HashSet<String> = palettes
        .iter()
        .map(|palette| palette.palette().name.clone())
        .collect();
    let entries = loader::list_available()?;
    let loader_names: HashSet<String> = entries
        .into_iter()
        .map(|entry| entry.theme.metadata.name.clone())
        .collect();
    for expected in expected_names {
        assert!(
            loader_names.contains(&expected),
            "loader missing palette '{}'",
            expected
        );
    }
    Ok(())
}
