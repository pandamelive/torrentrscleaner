mod error;
mod models;
mod repo;
mod service;
mod downloader;
mod engine;
mod scan_worker;
mod api;

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let data_dir = PathBuf::from("./data");
    std::fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join("app.redb");

    let repo = repo::AppRepo::open(&db_path)?;

    // Worker Runtime：扫描任务
    // start_scan_worker returns (Sender, Receiver) synchronously — do not await
    let (scan_tx, mut scan_rx) = scan_worker::worker_runtime::start_scan_worker(repo.clone());

    let shared_event = Arc::new(Mutex::new(None));
    let evt_clone = shared_event.clone();
    tokio::spawn(async move {
        while let Some(e) = scan_rx.recv().await {
            *evt_clone.lock().await = Some(e);
        }
    });

    // HTTP Runtime：Web接口

    let app_state = api::handlers::AppState {
        repo: repo.clone(),
        scan_cmd_tx: Arc::new(scan_tx),
        scan_event: shared_event,
    };

    let router = api::build_router(app_state);
    println!("torrent‑rs‑cleaner listen 0.0.0.0:8090");

    // use hyper::Server explicitly
    let addr = "0.0.0.0:8090".parse().unwrap();
    hyper::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
