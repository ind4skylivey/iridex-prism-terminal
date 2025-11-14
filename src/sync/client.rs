use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::error::PrismResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncData {
    pub themes: Vec<String>,
    pub config: serde_json::Value,
    pub dotfiles: Vec<String>,
    pub timestamp: String,
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
}

impl SyncClient {
    pub fn new(endpoint: Option<String>) -> PrismResult<Self> {
        Ok(Self {
            endpoint: endpoint.unwrap_or_else(|| "https://sync.iridex.invalid".into()),
            http: reqwest::Client::new(),
        })
    }

    pub async fn push(&self, payload: SyncData) -> PrismResult<()> {
        log::info!(
            "sync push to {} with {} themes",
            self.endpoint,
            payload.themes.len()
        );
        let _ = self
            .http
            .post(format!("{}/push", self.endpoint))
            .json(&payload)
            .send()
            .await?;
        Ok(())
    }

    pub async fn pull(&self) -> PrismResult<SyncData> {
        log::info!("sync pull from {}", self.endpoint);
        let resp = self
            .http
            .get(format!("{}/pull", self.endpoint))
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
            .http
            .get(format!("{}/status", self.endpoint))
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
}
