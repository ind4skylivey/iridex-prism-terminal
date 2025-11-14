use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::error::PrismResult;
use crate::sync::jwt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncData {
    pub themes: Vec<String>,
    pub config: serde_json::Value,
    pub dotfiles: Vec<DotfileRecord>,
    pub timestamp: String,
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
}

#[derive(Clone)]
pub struct SyncClient {
    endpoint: String,
    http: reqwest::Client,
    token: Option<String>,
}

impl SyncClient {
    pub fn new(
        endpoint: Option<String>,
        token: Option<String>,
        secret: Option<String>,
    ) -> PrismResult<Self> {
        if let Some(token) = token.as_ref() {
            jwt::validate(token, secret.as_deref())?;
        }
        Ok(Self {
            endpoint: endpoint.unwrap_or_else(|| "https://sync.iridex.invalid".into()),
            http: reqwest::Client::new(),
            token,
        })
    }

    pub async fn push(&self, payload: SyncData) -> PrismResult<()> {
        log::info!(
            "sync push to {} with {} themes",
            self.endpoint,
            payload.themes.len()
        );
        let _ = self
            .with_auth(
                self.http
                    .post(format!("{}/push", self.endpoint))
                    .json(&payload),
            )
            .send()
            .await?;
        Ok(())
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
        let resp = self
            .with_auth(self.http.get(format!("{}/status", self.endpoint)))
            .send()
            .await;
        let remote_timestamp = match resp {
            Ok(response) => match response.json::<SyncStatus>().await {
                Ok(status) => status.remote_timestamp,
                Err(_) => None,
            },
            Err(_) => None,
        };
        Ok(SyncStatus {
            local_timestamp: Local::now().to_rfc3339(),
            remote_timestamp,
        })
    }

    fn empty_payload() -> SyncData {
        SyncData {
            themes: Vec::new(),
            config: serde_json::json!({}),
            dotfiles: Vec::new(),
            timestamp: Local::now().to_rfc3339(),
        }
    }

    fn with_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token {
            builder.bearer_auth(token)
        } else {
            builder
        }
    }
}
