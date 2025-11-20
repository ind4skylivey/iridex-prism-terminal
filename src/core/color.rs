use serde::{Deserialize, Serialize};

use crate::error::{PrismError, PrismResult};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BrightPalette {
    #[serde(default)]
    pub black: String,
    #[serde(default)]
    pub red: String,
    #[serde(default)]
    pub green: String,
    #[serde(default)]
    pub yellow: String,
    #[serde(default)]
    pub blue: String,
    #[serde(default)]
    pub magenta: String,
    #[serde(default)]
    pub cyan: String,
    #[serde(default)]
    pub white: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorPalette {
    pub background: String,
    pub foreground: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    #[serde(default)]
    pub bright: BrightPalette,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: "#000000".into(),
            foreground: "#ffffff".into(),
            black: "#000000".into(),
            red: "#ff5555".into(),
            green: "#50fa7b".into(),
            yellow: "#f1fa8c".into(),
            blue: "#bd93f9".into(),
            magenta: "#ff79c6".into(),
            cyan: "#8be9fd".into(),
            white: "#bbbbbb".into(),
            bright: BrightPalette::default(),
        }
    }
}

impl ColorPalette {
    pub fn validate(&self) -> PrismResult<()> {
        for (label, value) in self.to_pairs() {
            if !value.starts_with('#') || value.len() < 4 {
                return Err(PrismError::new(format!(
                    "invalid color '{value}' for {label}; expected hex like #ffeeaa"
                )));
            }
        }
        Ok(())
    }

    fn to_pairs(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("background", &self.background),
            ("foreground", &self.foreground),
            ("black", &self.black),
            ("red", &self.red),
            ("green", &self.green),
            ("yellow", &self.yellow),
            ("blue", &self.blue),
            ("magenta", &self.magenta),
            ("cyan", &self.cyan),
            ("white", &self.white),
            ("bright.black", &self.bright.black),
            ("bright.red", &self.bright.red),
            ("bright.green", &self.bright.green),
            ("bright.yellow", &self.bright.yellow),
            ("bright.blue", &self.bright.blue),
            ("bright.magenta", &self.bright.magenta),
            ("bright.cyan", &self.bright.cyan),
            ("bright.white", &self.bright.white),
        ]
    }
}
