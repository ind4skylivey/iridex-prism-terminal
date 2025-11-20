use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::error::{PrismError, PrismResult};

/// Canonical identifiers for every built-in theme plus the user `Custom` slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeId {
    AuroraEdge,
    CyberNoir,
    Error808,
    ForestFlux,
    GlitchGrid,
    LavenderCore,
    MatrixShade,
    MidnightWarp,
    MonoQuiet,
    NebulaMocha,
    SynthwaveVoid,
    TerminalGhost,
    TokyoGhost,
    ObsidianForge,
    ArchVortex,
    QuantumJade,
    EclipseProtocol,
    SakuraSteel,
    ThemeTemplate,
    Custom,
}

impl ThemeId {
    pub const fn all() -> &'static [ThemeId; 20] {
        &[
            ThemeId::AuroraEdge,
            ThemeId::CyberNoir,
            ThemeId::Error808,
            ThemeId::ForestFlux,
            ThemeId::GlitchGrid,
            ThemeId::LavenderCore,
            ThemeId::MatrixShade,
            ThemeId::MidnightWarp,
            ThemeId::MonoQuiet,
            ThemeId::NebulaMocha,
            ThemeId::SynthwaveVoid,
            ThemeId::TerminalGhost,
            ThemeId::TokyoGhost,
            ThemeId::ObsidianForge,
            ThemeId::ArchVortex,
            ThemeId::QuantumJade,
            ThemeId::EclipseProtocol,
            ThemeId::SakuraSteel,
            ThemeId::ThemeTemplate,
            ThemeId::Custom,
        ]
    }

    pub fn slug(self) -> &'static str {
        match self {
            ThemeId::AuroraEdge => "aurora-edge",
            ThemeId::CyberNoir => "cyber-noir",
            ThemeId::Error808 => "error_808",
            ThemeId::ForestFlux => "forest-flux",
            ThemeId::GlitchGrid => "glitch-grid",
            ThemeId::LavenderCore => "lavender-core",
            ThemeId::MatrixShade => "matrix-shade",
            ThemeId::MidnightWarp => "midnight-warp",
            ThemeId::MonoQuiet => "mono-quiet",
            ThemeId::NebulaMocha => "nebula-mocha",
            ThemeId::SynthwaveVoid => "synthwave-void",
            ThemeId::TerminalGhost => "terminal-ghost",
            ThemeId::TokyoGhost => "tokyo-ghost",
            ThemeId::ObsidianForge => "obsidian-forge",
            ThemeId::ArchVortex => "arch-vortex",
            ThemeId::QuantumJade => "quantum-jade",
            ThemeId::EclipseProtocol => "eclipse-protocol",
            ThemeId::SakuraSteel => "sakura-steel",
            ThemeId::ThemeTemplate => "theme-template",
            ThemeId::Custom => "custom",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            ThemeId::AuroraEdge => "Aurora-Edge",
            ThemeId::CyberNoir => "Cyber-Noir",
            ThemeId::Error808 => "ERROR_808",
            ThemeId::ForestFlux => "Forest-Flux",
            ThemeId::GlitchGrid => "Glitch-Grid",
            ThemeId::LavenderCore => "Lavender-Core",
            ThemeId::MatrixShade => "Matrix-Shade",
            ThemeId::MidnightWarp => "Midnight-Warp",
            ThemeId::MonoQuiet => "Mono-Quiet",
            ThemeId::NebulaMocha => "Nebula-Mocha",
            ThemeId::SynthwaveVoid => "Synthwave-Void",
            ThemeId::TerminalGhost => "Terminal-Ghost",
            ThemeId::TokyoGhost => "Tokyo-Ghost",
            ThemeId::ObsidianForge => "Obsidian-Forge",
            ThemeId::ArchVortex => "Arch-Vortex",
            ThemeId::QuantumJade => "Quantum-Jade",
            ThemeId::EclipseProtocol => "Eclipse-Protocol",
            ThemeId::SakuraSteel => "Sakura-Steel",
            ThemeId::ThemeTemplate => "Theme-Template",
            ThemeId::Custom => "Custom",
        }
    }

    pub fn order_index(self) -> usize {
        Self::all()
            .iter()
            .position(|candidate| *candidate == self)
            .unwrap_or(usize::MAX)
    }

    pub fn from_slug(value: &str) -> Option<Self> {
        let normalized = normalize_theme_key(value);
        Self::all()
            .iter()
            .copied()
            .find(|candidate| normalize_theme_key(candidate.slug()) == normalized)
    }
}

impl fmt::Display for ThemeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone)]
pub struct ThemeMeta {
    pub id: ThemeId,
    pub name: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub description: String,
    pub recommended_terminal: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub base: Vec<String>,
    pub accents: Vec<String>,
    pub background: String,
    pub foreground: Option<String>,
}

impl Palette {
    pub fn new(
        mut base: Vec<String>,
        mut accents: Vec<String>,
        background: String,
        foreground: Option<String>,
    ) -> PrismResult<Self> {
        if base.is_empty() {
            return Err(PrismError::new(
                "palette must define at least one base color",
            ));
        }
        base = dedup(base);
        if base.len() < 16 {
            let snapshot = base.clone();
            for index in base.len()..16 {
                let seed = snapshot[index % snapshot.len()].clone();
                base.push(seed);
            }
        }
        if base.len() != 16 {
            return Err(PrismError::new(format!(
                "palette expected 16 base colors, found {}",
                base.len()
            )));
        }

        accents = dedup(accents);
        if accents.is_empty() {
            accents = base.iter().take(3).cloned().collect();
        }

        validate_colors(&base)?;
        validate_colors(&accents)?;
        validate_color(&background)?;
        if let Some(color) = &foreground {
            validate_color(color)?;
        }

        Ok(Self {
            base,
            accents,
            background,
            foreground,
        })
    }
}

fn dedup(values: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for value in values {
        let key = normalize_theme_key(&value);
        if seen.insert(key) {
            result.push(value);
        }
    }
    result
}

fn validate_colors(colors: &[String]) -> PrismResult<()> {
    for color in colors {
        validate_color(color)?;
    }
    Ok(())
}

fn validate_color(value: &str) -> PrismResult<()> {
    let trimmed = value.trim();
    let without_hash = trimmed.strip_prefix('#').unwrap_or(trimmed);
    let len = without_hash.len();
    if !(len == 6 || len == 8) {
        return Err(PrismError::new(format!(
            "color '{value}' must use 6 or 8 hex digits"
        )));
    }
    if !without_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(PrismError::new(format!(
            "color '{value}' contains non-hex characters"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct ThemeScripts {
    pub zsh: ScriptAsset,
    pub bash: ScriptAsset,
    pub fish: ScriptAsset,
}

impl ThemeScripts {
    pub fn new(theme_root: &Path, slug: &str) -> Self {
        Self {
            zsh: script_asset(theme_root, slug, "zsh", ".zsh-theme"),
            bash: script_asset(theme_root, slug, "bash", ".sh"),
            fish: script_asset(theme_root, slug, "fish", ".fish"),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &ScriptAsset)> {
        [
            ("zsh", &self.zsh),
            ("bash", &self.bash),
            ("fish", &self.fish),
        ]
        .into_iter()
    }
}

fn script_asset(theme_root: &Path, slug: &str, dir: &str, ext: &str) -> ScriptAsset {
    let path = theme_root.join(dir).join(format!("{slug}{ext}"));
    if path.exists() {
        ScriptAsset::File(path)
    } else {
        let template_name = match ext {
            ".zsh-theme" => "theme-template.zsh-theme",
            ".fish" => "theme-template.fish",
            _ => "theme-template.sh",
        };
        ScriptAsset::Template(theme_root.join("skeleton").join(template_name))
    }
}

#[derive(Debug, Clone)]
pub enum ScriptAsset {
    File(PathBuf),
    Template(PathBuf),
}

impl ScriptAsset {
    pub fn path(&self) -> &Path {
        match self {
            ScriptAsset::File(path) | ScriptAsset::Template(path) => path,
        }
    }

    pub fn kind(&self) -> ScriptAssetKind {
        match self {
            ScriptAsset::File(_) => ScriptAssetKind::File,
            ScriptAsset::Template(_) => ScriptAssetKind::Template,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptAssetKind {
    File,
    Template,
}

impl fmt::Display for ScriptAssetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptAssetKind::File => write!(f, "file"),
            ScriptAssetKind::Template => write!(f, "template"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub meta: ThemeMeta,
    pub palette: Palette,
    pub scripts: ThemeScripts,
}

/// Normalizes any user-provided theme key (slug, name, etc.).
pub fn normalize_theme_key(value: &str) -> String {
    value.trim().to_lowercase().replace(['_', ' '], "-")
}
