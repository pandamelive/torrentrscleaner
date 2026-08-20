use tokio::sync::mpsc;
use chrono::Utc;
use crate::service::msg::{ScanCommand, ScanEvent};
use crate::repo::AppRepo;
use crate::downloader::{DownloaderClient, QbDownloader};
use crate::models::{ScanResult, ScanLog, DownloaderType};
use crate::engine::precompile_rule::{compile_rules, apply_precompiled};
use crate::scan_worker::cache::TorrentCache;
use crate::scan_worker::batch_log::BatchLogWriter;
use crate::error::AppResult;

pub fn start_scan_worker(repo:AppRepo) -> (mpsc::Sender<ScanCommand>, mpsc::Receiver<ScanEvent>) {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<ScanCommand>(8);
    let (evt_tx, evt_rx) = mpsc::channel::<ScanEvent>(8);

    tokio::spawn(async move {
        let repo = repo;
        let mut cache = TorrentCache::new(10);
        let mut batch_log = BatchLogWriter::new(repo.clone());
        let mut predicates = Vec::new();
        let mut scan_running = false;
        let mut task_id:u64 = 1;

        let rules = repo.load_rules().unwrap_or_default();
        predicates = compile_rules(&rules);

        while let Some(cmd) = cmd_rx.recv().await {
            match cmd {
                ScanCommand::ReloadRules => {
                    let rules = repo.load_rules().unwrap_or_default();
                    predicates = compile_rules(&rules);
                }
                ScanCommand::ManualScan { force_refresh } => {
                    if scan_running {
                        let _ = evt_tx.send(ScanEvent::TaskError(task_id, "任务正在运行".into())).await;
                        continue;
                    }
                    scan_running = true;
                    let tid = task_id;
                    task_id +=1;
                    let _ = evt_tx.send(ScanEvent::TaskStarted(tid)).await;

                    let res:AppResult<ScanResult> = async{
                        let cfg = repo.load_config()?;
                        if force_refresh { cache.invalidate(); }

                        let downloader: Box<dyn DownloaderClient> = match &cfg.downloader_type {
                            DownloaderType::Qbittorrent => {
                                Box::new(QbDownloader::new(&cfg.addr, &cfg.username, &cfg.password).await?)
                            }
                        };

                        let list = if let Some(cached) = cache.get() {
                            cached.clone()
                        } else {
                            let fresh = downloader.fetch_all_preview().await?;
                            cache.set(fresh.clone());
                            fresh
                        };

                        let mut preview = list.clone();
                        apply_precompiled(&mut preview, &predicates);
                        let remove_cnt = preview.iter().filter(|x|x.will_remove).count();

                        let log = ScanLog {
                            time: Utc::now().timestamp_millis(),
                            dry_run: cfg.dry_run_global,
                            total_torrents: preview.len(),
                            to_remove: remove_cnt,
                            detail: format!("preview {} items", remove_cnt),
                        };

                        if !cfg.dry_run_global {
                            for item in &preview {
                                if item.will_remove {
                                    let _ = downloader.remove_torrent(&item.infohash, false).await;
                                }
                            }
                        }

                        batch_log.push(log.clone());
                        batch_log.flush()?;

                        Ok(ScanResult { task_id:tid, dry_run:cfg.dry_run_global, list:preview, log })
                    }.await;

                    match res {
                        Ok(r)=>{
                            let _ = evt_tx.send(ScanEvent::TaskFinished(r)).await;
                        }
                        Err(e)=>{
                            let _ = evt_tx.send(ScanEvent::TaskError(tid, e.to_string())).await;
                        }
                    }
                    scan_running = false;
                }
            }
        }
    });

    (cmd_tx, evt_rx)
}