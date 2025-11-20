use std::path::Path;
use colored::Colorize;

use crate::catalog::ThemeCatalog;
use crate::cli::resolve_theme;
use crate::cli::DevArgs; // reusing DevArgs since it's just { theme: String }
use crate::error::PrismResult;

pub fn handle_info(catalog: &ThemeCatalog, args: DevArgs) -> PrismResult<()> {
    let entry = resolve_theme(catalog, &args.theme)?;
    let theme = &entry.theme;
    let meta = &theme.meta;

    let dot_color = hex_to_rgb(&theme.palette.base[1]).unwrap_or((255, 255, 255));
    println!("\n{}  {}", "●".truecolor(dot_color.0, dot_color.1, dot_color.2), meta.name.bold().underline());
    println!("   {}", meta.slug.dimmed());

    if !meta.description.is_empty() {
        println!("\n{}", meta.description.italic());
    }

    println!("\n{}", "METADATA".bold());
    println!("  {:<12} {}", "ID:", format!("{:?}", meta.id).dimmed());
    if !meta.tags.is_empty() {
        println!("  {:<12} {}", "Tags:", meta.tags.join(", "));
    }
    if let Some(term) = &meta.recommended_terminal {
        println!("  {:<12} {}", "Terminal:", term);
    }
    
    println!("\n{}", "FILES".bold());
    println!("  {:<12} {}", "Palette:", entry.palette_path.display());
    // Check for docs
    let docs_path = find_docs_path(&entry.palette_path, &meta.slug);
    if let Some(path) = docs_path {
        println!("  {:<12} {}", "Docs:", path.display());
    } else {
        println!("  {:<12} {}", "Docs:", "None".dimmed());
    }

    println!("\n{}", "PALETTE PREVIEW".bold());
    // Show base colors
    print!("  Base:    ");
    for color in &theme.palette.base {
        print_swatch(color);
    }
    println!();
    
    // Show accents
    print!("  Accents: ");
    for color in &theme.palette.accents {
        print_swatch(color);
    }
    println!();

    println!("\nRun {} to apply this theme.", format!("prism apply {}", meta.slug).cyan());

    Ok(())
}

fn print_swatch(hex: &str) {
    // Parse hex to rgb for truecolor background
    if let Some((r, g, b)) = hex_to_rgb(hex) {
        print!("{}", "  ".on_truecolor(r, g, b));
    } else {
        print!("??");
    }
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn find_docs_path(palette_path: &Path, slug: &str) -> Option<std::path::PathBuf> {
    // Assuming docs are in ../../docs/<slug>.md relative to palette file
    // palette_path is .../themes/shared-palettes/<slug>.json
    // we want .../docs/<slug>.md
    
    // Go up two levels from palette file
    let root = palette_path.parent()?.parent()?; 
    let doc = root.join("docs").join(format!("{}.md", slug));
    if doc.exists() {
        Some(doc)
    } else {
        None
    }
}
