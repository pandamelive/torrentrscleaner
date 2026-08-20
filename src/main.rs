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

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let data_dir = PathBuf::from("./data");
    std::fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join("app.redb");

    let repo = repo::AppRepo::open(&db_path)?;

    // Worker Runtime：扫描任务
    let worker_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let (scan_tx, mut scan_rx) = worker_rt.block_on(async {
        scan_worker::worker_runtime::start_scan_worker(repo.clone())
    });

    let shared_event = Arc::new(Mutex::new(None));
    let evt_clone = shared_event.clone();
    worker_rt.spawn(async move {
        while let Some(e) = scan_rx.recv().await {
            *evt_clone.lock().await = Some(e);
        }
    });

    // HTTP Runtime：Web接口
    let http_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;

    let app_state = api::handlers::AppState {
        repo: repo.clone(),
        scan_cmd_tx: Arc::new(scan_tx),
        scan_event: shared_event,
    };

    let router = api::build_router(app_state);
    println!("torrent‑rs‑cleaner listen 0.0.0.0:8090");

    http_rt.block_on(async {
        axum::Server::bind(&"0.0.0.0:8090".parse().unwrap())
            .serve(router.into_make_svc())
            .await
    })?;

    Ok(())
}