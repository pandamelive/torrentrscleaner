use axum::{Json, extract::State};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::repo::AppRepo;
use crate::service::msg::{ScanCommand,ScanEvent};
use crate::error::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub repo: AppRepo,
    pub scan_cmd_tx: Arc<tokio::sync::mpsc::Sender<ScanCommand>>,
    pub scan_event: Arc<Mutex<Option<ScanEvent>>>,
}

pub async fn get_config(State(st): State<AppState>) -> Json<crate::models::AppConfig> {
    Json(st.repo.load_config().unwrap_or_default())
}

pub async fn trigger_scan(State(st): State<AppState>) -> AppResult<Json<String>> {
    st.scan_cmd_tx.send(ScanCommand::ManualScan{force_refresh:true}).await.map_err(|e|crate::error::AppError::Downloader(anyhow::anyhow!(e)))?;
    Ok(Json("scan task submitted".into()))
}