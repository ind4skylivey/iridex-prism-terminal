use std::future::Future;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::extract::State;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{serve, Json, Router};
use chrono::Local;
use serde_json::json;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::error::{PrismError, PrismResult};
use crate::metadata_dir;
use crate::sync::client::{SyncData, SyncStatus};
use crate::sync::jwt;

const STORAGE_FILE: &str = "sync-backend.json";

#[derive(Clone)]
struct BackendState {
    secret: String,
    storage_path: PathBuf,
    snapshot: Arc<RwLock<Option<SyncData>>>,
}

impl BackendState {
    fn load(secret: String) -> PrismResult<Self> {
        let path = metadata_dir()?.join(STORAGE_FILE);
        let snapshot = read_snapshot(&path)?;
        Ok(Self {
            secret,
            storage_path: path,
            snapshot: Arc::new(RwLock::new(snapshot)),
        })
    }

    fn secret(&self) -> &str {
        &self.secret
    }

    async fn save_snapshot(&self, payload: SyncData) -> PrismResult<()> {
        {
            let mut guard = self.snapshot.write().await;
            *guard = Some(payload.clone());
        }
        self.persist(&payload).await
    }

    async fn current_snapshot(&self) -> Option<SyncData> {
        self.snapshot.read().await.clone()
    }

    async fn persist(&self, payload: &SyncData) -> PrismResult<()> {
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let serialized = serde_json::to_string_pretty(payload)?;
        fs::write(&self.storage_path, serialized).await?;
        Ok(())
    }
}

pub async fn serve_until_ctrl_c(addr: SocketAddr, secret: String) -> PrismResult<()> {
    let listener = TcpListener::bind(addr).await?;
    serve_with_listener(listener, secret, async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await
}

pub async fn serve_with_listener<F>(
    listener: TcpListener,
    secret: String,
    shutdown: F,
) -> PrismResult<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let state = BackendState::load(secret)?;
    let app = Router::new()
        .route("/push", post(handle_push))
        .route("/pull", get(handle_pull))
        .route("/status", get(handle_status))
        .with_state(state);

    serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(|err| PrismError::new(err.to_string()))
}

async fn handle_push(
    State(state): State<BackendState>,
    headers: HeaderMap,
    Json(payload): Json<SyncData>,
) -> Result<Json<serde_json::Value>, ApiError> {
    authenticate(&state, &headers)?;
    state.save_snapshot(payload).await?;
    Ok(Json(json!({ "status": "ok" })))
}

async fn handle_pull(
    State(state): State<BackendState>,
    headers: HeaderMap,
) -> Result<Json<SyncData>, ApiError> {
    authenticate(&state, &headers)?;
    let snapshot = state
        .current_snapshot()
        .await
        .unwrap_or_else(empty_snapshot);
    Ok(Json(snapshot))
}

async fn handle_status(
    State(state): State<BackendState>,
    headers: HeaderMap,
) -> Result<Json<SyncStatus>, ApiError> {
    authenticate(&state, &headers)?;
    let (remote_timestamp, remote_version) = state
        .current_snapshot()
        .await
        .map(|snapshot| (Some(snapshot.timestamp), snapshot.version))
        .unwrap_or((None, None));
    Ok(Json(SyncStatus {
        local_timestamp: Local::now().to_rfc3339(),
        remote_timestamp,
        remote_version,
    }))
}

fn authenticate(state: &BackendState, headers: &HeaderMap) -> Result<(), ApiError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|raw| raw.strip_prefix("Bearer "))
        .map(|value| value.trim());
    let token = token.ok_or_else(|| ApiError::unauthorized("Missing bearer token"))?;
    jwt::validate(token, Some(state.secret()))
        .map(|_| ())
        .map_err(ApiError::from)
}

fn empty_snapshot() -> SyncData {
    SyncData {
        themes: Vec::new(),
        config: serde_json::json!({}),
        dotfiles: Vec::new(),
        timestamp: Local::now().to_rfc3339(),
        version: None,
    }
}

fn read_snapshot(path: &Path) -> PrismResult<Option<SyncData>> {
    if !path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(path)?;
    let snapshot = serde_json::from_str(&contents)?;
    Ok(Some(snapshot))
}

#[derive(Debug)]
enum ApiError {
    Unauthorized(String),
    Internal(PrismError),
}

impl ApiError {
    fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized(message.into())
    }
}

impl From<PrismError> for ApiError {
    fn from(value: PrismError) -> Self {
        ApiError::Internal(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message).into_response(),
            ApiError::Internal(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
        }
    }
}
