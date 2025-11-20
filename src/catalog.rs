use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::error::{PrismError, PrismResult};
use crate::themes::{normalize_theme_key, Palette, Theme, ThemeId, ThemeMeta, ThemeScripts};
use crate::themes_root;

#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub theme: Theme,
    pub palette_path: PathBuf,
}

#[derive(Debug, Default)]
pub struct ThemeCatalog {
    entries: Vec<CatalogEntry>,
    slug_index: HashMap<String, usize>,
    id_index: HashMap<ThemeId, usize>,
}

impl ThemeCatalog {
    pub fn iter(&self) -> impl Iterator<Item = &CatalogEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn resolve(&self, query: &str) -> Option<&CatalogEntry> {
        let key = normalize_theme_key(query);
        self.slug_index.get(&key).map(|index| &self.entries[*index])
    }

    pub fn get(&self, id: ThemeId) -> Option<&CatalogEntry> {
        self.id_index.get(&id).map(|index| &self.entries[*index])
    }
}

pub fn load_catalog() -> PrismResult<ThemeCatalog> {
    let theme_root = themes_root()?;
    let palette_dir = theme_root.join("shared-palettes");
    if !palette_dir.exists() {
        return Err(PrismError::new(format!(
            "missing palette directory: {}",
            palette_dir.display()
        )));
    }
    ensure_custom_palette(&palette_dir)?;

    let mut entries = Vec::new();
    for entry in fs::read_dir(&palette_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let slug = match path.file_stem().and_then(|stem| stem.to_str()) {
            Some(value) => value.to_lowercase(),
            None => continue,
        };
        let id = ThemeId::from_slug(&slug).ok_or_else(|| {
            PrismError::new(format!("unknown theme slug '{slug}' in {}", path.display()))
        })?;
        let raw = fs::read_to_string(&path)?;
        let value: Value = serde_json::from_str(&raw)?;
        let palette = parse_palette(&value)?;
        let tags = extract_tags(&value);
        let description = value
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let recommended_terminal = value
            .get("recommended_terminal")
            .or_else(|| value.get("recommendedTerminal"))
            .and_then(Value::as_str)
            .map(|value| value.to_string());
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .map(|value| value.to_string())
            .unwrap_or_else(|| id.display_name().to_string());

        let meta = ThemeMeta {
            id,
            name,
            slug: slug.clone(),
            tags,
            description,
            recommended_terminal,
        };
        let scripts = ThemeScripts::new(&theme_root, &slug);
        entries.push(CatalogEntry {
            theme: Theme {
                meta,
                palette,
                scripts,
            },
            palette_path: path,
        });
    }

    entries.sort_by(|a, b| {
        a.theme
            .meta
            .id
            .order_index()
            .cmp(&b.theme.meta.id.order_index())
            .then_with(|| a.theme.meta.slug.cmp(&b.theme.meta.slug))
    });

    let mut catalog = ThemeCatalog::default();
    for (index, entry) in entries.into_iter().enumerate() {
        insert_indices(&mut catalog.slug_index, &entry, index);
        catalog.id_index.insert(entry.theme.meta.id, index);
        catalog.entries.push(entry);
    }
    Ok(catalog)
}

fn insert_indices(map: &mut HashMap<String, usize>, entry: &CatalogEntry, index: usize) {
    let slug = normalize_theme_key(&entry.theme.meta.slug);
    map.insert(slug.clone(), index);
    map.insert(
        normalize_theme_key(&entry.theme.meta.slug.replace('_', "-")),
        index,
    );
    map.insert(normalize_theme_key(&entry.theme.meta.name), index);
}

fn ensure_custom_palette(dir: &Path) -> PrismResult<()> {
    let custom = dir.join("custom.json");
    if custom.exists() {
        return Ok(());
    }
    let template = dir.join("theme-template.json");
    if template.exists() {
        let data = fs::read_to_string(&template)?;
        let mut value: Value = serde_json::from_str(&data)?;
        if let Value::Object(map) = &mut value {
            map.insert("name".into(), Value::String("Custom".into()));
            map.insert(
                "description".into(),
                Value::String("User-defined Prism palette".into()),
            );
        }
        fs::write(&custom, serde_json::to_string_pretty(&value)?)?;
        return Ok(());
    }

    let fallback = json!({
        "name": "Custom",
        "description": "User-defined Prism palette",
        "bg": "#0b0b0b",
        "fg": "#f8f8f2",
        "primary": "#ff79c6",
        "secondary": "#8be9fd",
        "accent": "#bd93f9",
        "error": "#ff5555",
        "success": "#50fa7b"
    });
    fs::write(&custom, serde_json::to_string_pretty(&fallback)?)?;
    Ok(())
}

fn parse_palette(value: &Value) -> PrismResult<Palette> {
    let mut base = string_array(value, &["base", "base16", "base_colors"]);
    if base.is_empty() {
        base = collect_colors(
            value,
            &[
                "bg",
                "fg",
                "primary",
                "secondary",
                "accent",
                "error",
                "success",
            ],
        );
    }
    let mut accents = string_array(value, &["accents", "accentColors"]);
    if accents.is_empty() {
        accents = collect_colors(value, &["primary", "secondary", "accent"]);
    }
    let background = string_field(value, &["background", "bg"]).unwrap_or_else(|| "#000000".into());
    let foreground = string_field(value, &["foreground", "fg"]);
    Palette::new(base, accents, background, foreground)
}

fn string_array(value: &Value, keys: &[&str]) -> Vec<String> {
    for key in keys {
        if let Some(array) = value.get(*key).and_then(Value::as_array) {
            let collected: Vec<String> = array
                .iter()
                .filter_map(Value::as_str)
                .map(|s| s.to_string())
                .collect();
            if !collected.is_empty() {
                return collected;
            }
        }
    }
    Vec::new()
}

fn collect_colors(value: &Value, keys: &[&str]) -> Vec<String> {
    keys.iter()
        .filter_map(|key| value.get(*key).and_then(Value::as_str))
        .map(|s| s.to_string())
        .collect()
}

fn string_field(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|key| value.get(*key).and_then(Value::as_str))
        .map(|s| s.to_string())
        .next()
}

fn extract_tags(value: &Value) -> Vec<String> {
    let mut tags = string_array(value, &["tags", "mood_tags"]);
    if tags.is_empty() {
        if let Some(tag) = value
            .get("recommended_terminal")
            .or_else(|| value.get("recommendedTerminal"))
            .and_then(Value::as_str)
        {
            tags.push(tag.to_string());
        }
    }
    tags
}
