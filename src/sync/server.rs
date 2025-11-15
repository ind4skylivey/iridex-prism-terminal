use std::collections::{BTreeMap, BTreeSet};
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
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::error::{PrismError, PrismResult};
use crate::metadata_dir;
use crate::sync::client::{DotfileRecord, SyncData, SyncPushRequest, SyncPushResponse, SyncStatus};
use crate::sync::jwt;

const STORAGE_FILE: &str = "sync-backend.json";
const MAX_HISTORY: usize = 50;

#[derive(Clone)]
struct BackendState {
    secret: String,
    storage_path: PathBuf,
    data: Arc<RwLock<PersistedStore>>,
}

impl BackendState {
    fn load(secret: String) -> PrismResult<Self> {
        let path = metadata_dir()?.join(STORAGE_FILE);
        let store = read_store(&path)?;
        Ok(Self {
            secret,
            storage_path: path,
            data: Arc::new(RwLock::new(store)),
        })
    }

    fn secret(&self) -> &str {
        &self.secret
    }

    async fn current_snapshot(&self) -> Option<SyncData> {
        let guard = self.data.read().await;
        guard.current().map(|snapshot| {
            let mut payload = snapshot.payload.clone();
            payload.version = Some(snapshot.version);
            payload
        })
    }

    async fn apply_push(&self, request: SyncPushRequest) -> Result<VersionedSnapshot, ApiError> {
        let mut guard = self.data.write().await;
        let next_version = guard.next_version;
        let base = match request.base_version {
            Some(version) => guard.find(version).cloned().ok_or_else(|| {
                ApiError::conflict(format!("Unknown base version {version}"), None)
            })?,
            None => {
                if guard.current().is_some() {
                    return Err(ApiError::conflict(
                        "Missing base_version. Pull before pushing new changes.",
                        None,
                    ));
                }
                VersionedSnapshot {
                    version: 0,
                    payload: empty_snapshot(),
                }
            }
        };

        let mut incoming = request.payload;
        if incoming.timestamp.trim().is_empty() {
            incoming.timestamp = Local::now().to_rfc3339();
        }

        let current_payload = guard
            .current()
            .map(|snapshot| snapshot.payload.clone())
            .unwrap_or_else(empty_snapshot);
        let delta = compute_delta(&base.payload, &incoming);
        let conflicts = detect_conflicts(&current_payload, &base.payload, &delta);
        if !conflicts.is_empty() {
            return Err(ApiError::conflict(
                "Conflicting changes detected",
                Some(conflicts),
            ));
        }

        let mut merged = current_payload;
        apply_delta(&mut merged, &delta);
        merged.timestamp = incoming.timestamp.clone();
        merged.version = Some(next_version);

        let snapshot = VersionedSnapshot {
            version: next_version,
            payload: merged.clone(),
        };
        let delta_entry = SnapshotDeltaEntry {
            version: next_version,
            parent: Some(base.version).filter(|v| *v != 0),
            timestamp: merged.timestamp.clone(),
            delta,
        };
        guard.push(snapshot.clone(), delta_entry);
        let serialized = serde_json::to_string_pretty(&*guard)
            .map_err(|err| ApiError::from(PrismError::new(err.to_string())))?;
        drop(guard);
        self.persist_serialized(&serialized).await?;
        Ok(snapshot)
    }

    async fn persist_serialized(&self, payload: &str) -> PrismResult<()> {
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&self.storage_path, payload).await?;
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
    Json(payload): Json<SyncPushRequest>,
) -> Result<Json<SyncPushResponse>, ApiError> {
    authenticate(&state, &headers)?;
    let snapshot = state.apply_push(payload).await?;
    let timestamp = snapshot.payload.timestamp.clone();
    Ok(Json(SyncPushResponse {
        version: snapshot.version,
        timestamp,
    }))
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

#[derive(Debug)]
enum ApiError {
    Unauthorized(String),
    Conflict {
        message: String,
        conflicts: Option<ConflictSummary>,
    },
    Internal(PrismError),
}

impl ApiError {
    fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized(message.into())
    }

    fn conflict(message: impl Into<String>, conflicts: Option<ConflictSummary>) -> Self {
        Self::Conflict {
            message: message.into(),
            conflicts,
        }
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
            ApiError::Conflict { message, conflicts } => {
                let payload = json!({
                    "error": "conflict",
                    "message": message,
                    "conflicts": conflicts,
                });
                (StatusCode::CONFLICT, Json(payload)).into_response()
            }
            ApiError::Internal(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
        }
    }
}

fn read_store(path: &Path) -> PrismResult<PersistedStore> {
    if !path.exists() {
        return Ok(PersistedStore::default());
    }
    let contents = std::fs::read_to_string(path)?;
    if let Ok(store) = serde_json::from_str::<PersistedStore>(&contents) {
        return Ok(store.normalize());
    }
    let mut legacy: SyncData = serde_json::from_str(&contents)?;
    legacy.version = Some(1);
    Ok(PersistedStore::from_legacy(legacy))
}

pub(crate) fn backend_store_path() -> PrismResult<PathBuf> {
    Ok(metadata_dir()?.join(STORAGE_FILE))
}

pub(crate) fn load_persisted_store() -> PrismResult<PersistedStore> {
    let path = backend_store_path()?;
    read_store(&path)
}

pub(crate) fn save_persisted_store(store: &PersistedStore) -> PrismResult<()> {
    let path = backend_store_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(store)?)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct VersionedSnapshot {
    pub(crate) version: u64,
    pub(crate) payload: SyncData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotDeltaEntry {
    pub(crate) version: u64,
    pub(crate) parent: Option<u64>,
    pub(crate) timestamp: String,
    pub(crate) delta: SnapshotDelta,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct SnapshotDelta {
    pub(crate) themes_added: Vec<String>,
    pub(crate) themes_removed: Vec<String>,
    pub(crate) dotfiles_upserted: Vec<DotfileRecord>,
    pub(crate) dotfiles_removed: Vec<String>,
    pub(crate) config_updates: Vec<ConfigUpdate>,
    pub(crate) config_removed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ConfigUpdate {
    pub(crate) key: String,
    pub(crate) value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ConflictSummary {
    dotfiles: Vec<String>,
    config: Vec<String>,
}

impl ConflictSummary {
    fn is_empty(&self) -> bool {
        self.dotfiles.is_empty() && self.config.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PersistedStore {
    versions: Vec<VersionedSnapshot>,
    deltas: Vec<SnapshotDeltaEntry>,
    next_version: u64,
}

impl Default for PersistedStore {
    fn default() -> Self {
        Self {
            versions: Vec::new(),
            deltas: Vec::new(),
            next_version: 1,
        }
    }
}

impl PersistedStore {
    pub(crate) fn current(&self) -> Option<&VersionedSnapshot> {
        self.versions.last()
    }

    pub(crate) fn find(&self, version: u64) -> Option<&VersionedSnapshot> {
        self.versions.iter().find(|entry| entry.version == version)
    }

    fn push(&mut self, snapshot: VersionedSnapshot, delta: SnapshotDeltaEntry) {
        if self.versions.len() >= MAX_HISTORY {
            self.versions.remove(0);
        }
        if self.deltas.len() >= MAX_HISTORY {
            self.deltas.remove(0);
        }
        self.versions.push(snapshot);
        self.deltas.push(delta);
        self.next_version = self
            .versions
            .last()
            .map(|entry| entry.version + 1)
            .unwrap_or(1);
    }

    fn normalize(mut self) -> Self {
        if self.next_version == 0 {
            self.next_version = self
                .versions
                .last()
                .map(|entry| entry.version + 1)
                .unwrap_or(1);
        }
        self
    }

    fn from_legacy(payload: SyncData) -> Self {
        let snapshot = VersionedSnapshot {
            version: 1,
            payload,
        };
        Self {
            versions: vec![snapshot],
            deltas: Vec::new(),
            next_version: 2,
        }
    }
    pub(crate) fn versions(&self) -> &[VersionedSnapshot] {
        &self.versions
    }

    pub(crate) fn prune_to_latest(&mut self, keep: usize) {
        if keep == 0 {
            self.versions.clear();
            self.deltas.clear();
            self.next_version = 1;
            return;
        }
        if self.versions.len() > keep {
            let drop_count = self.versions.len() - keep;
            let min_version = self.versions[drop_count].version;
            self.versions.drain(0..drop_count);
            self.deltas.retain(|entry| entry.version >= min_version);
        }
        self.next_version = self
            .versions
            .last()
            .map(|entry| entry.version + 1)
            .unwrap_or(1);
    }

    pub(crate) fn delta_for(&self, version: u64) -> Option<&SnapshotDeltaEntry> {
        self.deltas.iter().find(|entry| entry.version == version)
    }
}

fn compute_delta(base: &SyncData, target: &SyncData) -> SnapshotDelta {
    let mut delta = SnapshotDelta::default();
    let base_themes: BTreeSet<_> = base.themes.iter().cloned().collect();
    let target_themes: BTreeSet<_> = target.themes.iter().cloned().collect();
    delta.themes_added = target_themes.difference(&base_themes).cloned().collect();
    delta.themes_removed = base_themes.difference(&target_themes).cloned().collect();

    let base_dotfiles = dotfile_map(base);
    let target_dotfiles = dotfile_map(target);
    for (name, record) in &target_dotfiles {
        match base_dotfiles.get(name) {
            Some(existing) if dotfile_eq(existing, record) => {}
            _ => delta.dotfiles_upserted.push(record.clone()),
        }
    }
    for (name, record) in &base_dotfiles {
        if !target_dotfiles
            .get(name)
            .map(|new| dotfile_eq(new, record))
            .unwrap_or(false)
            && !target_dotfiles.contains_key(name)
        {
            delta.dotfiles_removed.push(name.clone());
        }
    }

    let base_config = config_map(base);
    let target_config = config_map(target);
    for (key, value) in &target_config {
        match base_config.get(key) {
            Some(existing) if existing == value => {}
            _ => delta.config_updates.push(ConfigUpdate {
                key: key.clone(),
                value: value.clone(),
            }),
        }
    }
    for key in base_config.keys() {
        if !target_config.contains_key(key) {
            delta.config_removed.push(key.clone());
        }
    }

    delta
}

fn apply_delta(target: &mut SyncData, delta: &SnapshotDelta) {
    let mut themes: BTreeSet<_> = target.themes.iter().cloned().collect();
    for add in &delta.themes_added {
        themes.insert(add.clone());
    }
    for remove in &delta.themes_removed {
        themes.remove(remove);
    }
    target.themes = themes.into_iter().collect();

    let mut config_obj = target.config.as_object().cloned().unwrap_or_default();
    for key in &delta.config_removed {
        config_obj.remove(key);
    }
    for update in &delta.config_updates {
        config_obj.insert(update.key.clone(), update.value.clone());
    }
    target.config = serde_json::Value::Object(config_obj);

    let mut dotfiles = dotfile_map(target);
    for key in &delta.dotfiles_removed {
        dotfiles.remove(key);
    }
    for record in &delta.dotfiles_upserted {
        dotfiles.insert(record.name.clone(), record.clone());
    }
    target.dotfiles = dotfiles.into_values().collect();
}

fn detect_conflicts(current: &SyncData, base: &SyncData, delta: &SnapshotDelta) -> ConflictSummary {
    let mut summary = ConflictSummary::default();
    let base_dotfiles = dotfile_map(base);
    let current_dotfiles = dotfile_map(current);

    for record in &delta.dotfiles_upserted {
        let base_entry = base_dotfiles.get(&record.name);
        let current_entry = current_dotfiles.get(&record.name);
        if !can_apply_dotfile_update(base_entry, current_entry, record) {
            summary.dotfiles.push(record.name.clone());
        }
    }
    for name in &delta.dotfiles_removed {
        let base_entry = base_dotfiles.get(name);
        let current_entry = current_dotfiles.get(name);
        if !can_apply_dotfile_removal(base_entry, current_entry) {
            summary.dotfiles.push(name.clone());
        }
    }

    let base_config = config_map(base);
    let current_config = config_map(current);
    for update in &delta.config_updates {
        let base_value = base_config.get(&update.key);
        let current_value = current_config.get(&update.key);
        if !can_apply_config_update(base_value, current_value, &update.value) {
            summary.config.push(update.key.clone());
        }
    }
    for key in &delta.config_removed {
        let base_value = base_config.get(key);
        let current_value = current_config.get(key);
        if !can_apply_config_removal(base_value, current_value) {
            summary.config.push(key.clone());
        }
    }

    summary
}

fn dotfile_map(snapshot: &SyncData) -> BTreeMap<String, DotfileRecord> {
    let mut map = BTreeMap::new();
    for record in &snapshot.dotfiles {
        map.insert(record.name.clone(), record.clone());
    }
    map
}

fn dotfile_eq(left: &DotfileRecord, right: &DotfileRecord) -> bool {
    match (left.sha256.as_ref(), right.sha256.as_ref()) {
        (Some(a), Some(b)) => a == b,
        _ => left.contents == right.contents,
    }
}

fn can_apply_dotfile_update(
    base: Option<&DotfileRecord>,
    current: Option<&DotfileRecord>,
    incoming: &DotfileRecord,
) -> bool {
    match (base, current) {
        (_, Some(curr)) if dotfile_eq(curr, incoming) => true,
        (Some(base_rec), Some(curr)) if dotfile_eq(base_rec, curr) => true,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(curr)) => dotfile_eq(curr, incoming),
        _ => false,
    }
}

fn can_apply_dotfile_removal(
    base: Option<&DotfileRecord>,
    current: Option<&DotfileRecord>,
) -> bool {
    match (base, current) {
        (Some(base_rec), Some(curr)) => dotfile_eq(base_rec, curr),
        (Some(_), None) => true,
        (None, _) => true,
    }
}

fn config_map(snapshot: &SyncData) -> BTreeMap<String, serde_json::Value> {
    snapshot
        .config
        .as_object()
        .map(|map| {
            map.iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn can_apply_config_update(
    base: Option<&serde_json::Value>,
    current: Option<&serde_json::Value>,
    incoming: &serde_json::Value,
) -> bool {
    if current == base {
        return true;
    }
    if let Some(curr) = current {
        return curr == incoming;
    }
    base.is_none()
}

fn can_apply_config_removal(
    base: Option<&serde_json::Value>,
    current: Option<&serde_json::Value>,
) -> bool {
    match (base, current) {
        (Some(base_val), Some(curr)) => curr == base_val,
        (Some(_), None) => true,
        (None, _) => true,
    }
}
