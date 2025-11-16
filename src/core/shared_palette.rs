use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::core::{
    color::{BrightPalette, ColorPalette},
    prompt::PromptStyle,
    theme::{ContextRules, PromptConfig, PromptSegment, Theme, ThemeMetadata, WidgetConfig},
};
use crate::error::{PrismError, PrismResult};

const DEFAULT_AUTHOR: &str = "Prism Terminal";
const DEFAULT_VERSION: &str = "0.1.1";

#[derive(Debug, Deserialize)]
pub struct SharedPalette {
    pub name: String,
    pub description: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub bg: String,
    pub fg: String,
    pub error: String,
    pub success: String,
}

#[derive(Debug)]
pub struct SharedPaletteEntry {
    slug: String,
    path: PathBuf,
    palette: SharedPalette,
}

impl SharedPaletteEntry {
    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn palette(&self) -> &SharedPalette {
        &self.palette
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn to_theme(&self) -> Theme {
        Theme {
            metadata: ThemeMetadata {
                name: self.palette.name.clone(),
                author: DEFAULT_AUTHOR.into(),
                version: DEFAULT_VERSION.into(),
                description: self.palette.description.clone(),
                tags: vec![self.slug.clone()],
            },
            colors: self.color_palette(),
            prompt: self.prompt_config(),
            widgets: WidgetConfig::default(),
            context_rules: ContextRules::default(),
        }
    }

    fn prompt_config(&self) -> PromptConfig {
        let mut segments = BTreeMap::new();
        segments.insert(
            "status".into(),
            PromptSegment {
                bg: self.palette.accent.clone(),
                fg: self.palette.bg.clone(),
                icon: Some("⚡".into()),
            },
        );
        segments.insert(
            "path".into(),
            PromptSegment {
                bg: self.palette.primary.clone(),
                fg: self.palette.bg.clone(),
                icon: Some("".into()),
            },
        );
        segments.insert(
            "git".into(),
            PromptSegment {
                bg: self.palette.secondary.clone(),
                fg: self.palette.bg.clone(),
                icon: Some("".into()),
            },
        );
        PromptConfig {
            style: PromptStyle::Powerline,
            show_user: false,
            show_host: false,
            show_time: true,
            show_git: true,
            separator: "".into(),
            segments,
            segment_order: vec!["status".into(), "path".into(), "git".into()],
        }
    }

    fn color_palette(&self) -> ColorPalette {
        ColorPalette {
            background: self.palette.bg.clone(),
            foreground: self.palette.fg.clone(),
            black: self.palette.bg.clone(),
            red: self.palette.error.clone(),
            green: self.palette.success.clone(),
            yellow: self.palette.accent.clone(),
            blue: self.palette.primary.clone(),
            magenta: self.palette.secondary.clone(),
            cyan: self.palette.accent.clone(),
            white: self.palette.fg.clone(),
            bright: BrightPalette {
                black: self.palette.bg.clone(),
                red: self.palette.error.clone(),
                green: self.palette.success.clone(),
                yellow: self.palette.accent.clone(),
                blue: self.palette.primary.clone(),
                magenta: self.palette.secondary.clone(),
                cyan: self.palette.accent.clone(),
                white: self.palette.fg.clone(),
            },
        }
    }
}

pub fn load_shared_palettes(root: &Path) -> PrismResult<Vec<SharedPaletteEntry>> {
    let mut entries = Vec::new();
    if !root.exists() {
        return Ok(entries);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let slug = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| {
                PrismError::new(format!("invalid palette filename {}", path.display()))
            })?;
        if slug == "theme-template" {
            continue;
        }
        let data = fs::read_to_string(&path)?;
        let palette: SharedPalette = serde_json::from_str(&data)
            .map_err(|err| PrismError::new(format!("failed to parse {}: {err}", path.display())))?;
        entries.push(SharedPaletteEntry {
            slug: slug.to_string(),
            path: path.clone(),
            palette,
        });
    }
    Ok(entries)
}
