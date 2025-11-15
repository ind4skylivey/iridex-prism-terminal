use std::path::PathBuf;

use chrono::Local;
use reqwest::tls::Certificate;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::error::{PrismError, PrismResult};
use crate::sync::jwt;

const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:7878";
const ENDPOINT_ENV: &str = "PRISM_SYNC_ENDPOINT";
const CA_BUNDLE_ENV: &str = "PRISM_SYNC_CA_BUNDLE";
const INSECURE_ENV: &str = "PRISM_SYNC_INSECURE";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncData {
    pub themes: Vec<String>,
    pub config: serde_json::Value,
    pub dotfiles: Vec<DotfileRecord>,
    pub timestamp: String,
    #[serde(default)]
    pub version: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotfileRecord {
    pub name: String,
    pub original: Option<String>,
    pub contents: String,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub modified: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub local_timestamp: String,
    pub remote_timestamp: Option<String>,
    #[serde(default)]
    pub remote_version: Option<u64>,
    #[serde(default)]
    pub remote_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPushRequest {
    pub base_version: Option<u64>,
    pub payload: SyncData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPushResponse {
    pub version: u64,
    pub timestamp: String,
}

#[derive(Clone)]
pub struct SyncClient {
    endpoint: String,
    http: Client,
    token: Option<String>,
}

impl SyncClient {
    pub fn new(
        options: ClientOptions,
        token: Option<String>,
        secret: Option<String>,
    ) -> PrismResult<Self> {
        if let Some(token) = token.as_ref() {
            jwt::validate(token, secret.as_deref())?;
        }
        let endpoint = options
            .endpoint
            .or_else(|| std::env::var(ENDPOINT_ENV).ok())
            .unwrap_or_else(|| DEFAULT_ENDPOINT.into());
        let ca_bundle = options
            .ca_bundle
            .or_else(|| std::env::var(CA_BUNDLE_ENV).ok().map(PathBuf::from));
        let danger_accept_invalid = options.danger_accept_invalid_certs
            || std::env::var(INSECURE_ENV)
                .ok()
                .map(|value| truthy(&value))
                .unwrap_or(false);

        let mut builder = Client::builder().danger_accept_invalid_certs(danger_accept_invalid);
        if let Some(path) = ca_bundle {
            let bytes = std::fs::read(&path).map_err(|err| {
                PrismError::new(format!(
                    "Failed to read CA bundle {}: {err}",
                    path.display()
                ))
            })?;
            let cert = load_certificate(&bytes).ok_or_else(|| {
                PrismError::new(format!(
                    "Unable to parse CA bundle at {} (expected PEM or DER)",
                    path.display()
                ))
            })?;
            builder = builder.add_root_certificate(cert);
        }

        let http = builder
            .build()
            .map_err(|err| PrismError::new(err.to_string()))?;
        Ok(Self {
            endpoint,
            http,
            token,
        })
    }

    pub async fn push(
        &self,
        payload: SyncData,
        base_version: Option<u64>,
    ) -> PrismResult<SyncPushResponse> {
        log::info!(
            "sync push to {} with {} themes (base {:?})",
            self.endpoint,
            payload.themes.len(),
            base_version
        );
        let request = SyncPushRequest {
            base_version,
            payload,
        };
        let response = self
            .with_auth(
                self.http
                    .post(format!("{}/push", self.endpoint))
                    .json(&request),
            )
            .send()
            .await?;
        if response.status() == StatusCode::CONFLICT {
            let details = response.json::<ConflictPayload>().await.ok();
            let message = details
                .as_ref()
                .and_then(|payload| payload.message.clone())
                .unwrap_or_else(|| "Sync conflict detected".into());
            let conflict_msg = details
                .and_then(|payload| payload.describe())
                .unwrap_or_default();
            return Err(PrismError::new(format!("{message}{conflict_msg}")));
        }
        let response = response.error_for_status()?;
        let payload = response.json::<SyncPushResponse>().await?;
        Ok(payload)
    }

    pub async fn pull(&self) -> PrismResult<SyncData> {
        log::info!("sync pull from {}", self.endpoint);
        let resp = self
            .with_auth(self.http.get(format!("{}/pull", self.endpoint)))
            .send()
            .await?;
        let data = resp
            .json::<SyncData>()
            .await
            .unwrap_or_else(|_| Self::empty_payload());
        Ok(data)
    }

    pub async fn status(&self) -> PrismResult<SyncStatus> {
        let local = Local::now().to_rfc3339();
        let request = self.with_auth(self.http.get(format!("{}/status", self.endpoint)));
        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<SyncStatus>().await {
                        Ok(mut status) => {
                            status.local_timestamp = local;
                            Ok(status)
                        }
                        Err(err) => Ok(SyncStatus {
                            local_timestamp: local,
                            remote_timestamp: None,
                            remote_version: None,
                            remote_error: Some(format!("Failed to parse status response: {err}")),
                        }),
                    }
                } else {
                    Ok(SyncStatus {
                        local_timestamp: local,
                        remote_timestamp: None,
                        remote_version: None,
                        remote_error: Some(format!("HTTP {}", response.status())),
                    })
                }
            }
            Err(err) => Ok(SyncStatus {
                local_timestamp: local,
                remote_timestamp: None,
                remote_version: None,
                remote_error: Some(err.to_string()),
            }),
        }
    }

    fn with_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token {
            builder.bearer_auth(token)
        } else {
            builder
        }
    }

    fn empty_payload() -> SyncData {
        SyncData {
            themes: Vec::new(),
            config: serde_json::json!({}),
            dotfiles: Vec::new(),
            timestamp: Local::now().to_rfc3339(),
            version: None,
        }
    }
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn load_certificate(bytes: &[u8]) -> Option<Certificate> {
    Certificate::from_pem(bytes)
        .ok()
        .or_else(|| Certificate::from_der(bytes).ok())
}

#[derive(Debug, Clone, Default)]
pub struct ClientOptions {
    pub endpoint: Option<String>,
    pub ca_bundle: Option<PathBuf>,
    pub danger_accept_invalid_certs: bool,
}

impl From<Option<String>> for ClientOptions {
    fn from(endpoint: Option<String>) -> Self {
        Self {
            endpoint,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConflictPayload {
    message: Option<String>,
    conflicts: Option<ConflictDetails>,
}

#[derive(Debug, Deserialize, Default)]
struct ConflictDetails {
    #[serde(default)]
    dotfiles: Vec<String>,
    #[serde(default)]
    config: Vec<String>,
}

impl ConflictPayload {
    fn describe(self) -> Option<String> {
        self.conflicts.and_then(|details| {
            let mut sections = Vec::new();
            if !details.dotfiles.is_empty() {
                sections.push(format!(" dotfiles [{}]", details.dotfiles.join(", ")));
            }
            if !details.config.is_empty() {
                sections.push(format!(" config keys [{}]", details.config.join(", ")));
            }
            if sections.is_empty() {
                None
            } else {
                Some(format!("\nConflicting fields:{}", sections.join(";")))
            }
        })
    }
}
