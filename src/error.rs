use thiserror::Error;

pub type PrismResult<T> = Result<T, PrismError>;

#[derive(Debug, Error)]
pub enum PrismError {
    #[error("{0}")]
    Message(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Theme parsing error: {0}")]
    ThemeParse(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Filesystem walk error: {0}")]
    Walkdir(#[from] walkdir::Error),
    #[error("System error: {0}")]
    System(String),
    #[error("Unsupported shell: {0}")]
    UnsupportedShell(String),
}

impl PrismError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}
