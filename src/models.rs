use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloaderType {
    Qbittorrent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub downloader_type: DownloaderType,
    pub addr: String,
    pub username: String,
    pub password: String,
    pub scan_interval_sec: u64,
    pub dry_run_global: bool,
    pub torrent_cache_ttl_sec: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            downloader_type: DownloaderType::Qbittorrent,
            addr: "http://127.0.0.1:8080".to_string(),
            username: "admin".to_string(),
            password: String::new(),
            scan_interval_sec: 3600,
            dry_run_global: true,
            torrent_cache_ttl_sec: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanRule {
    pub id: String,
    pub name: String,
    pub min_ratio: f64,
    pub min_seed_seconds: u64,
    pub min_free_disk_bytes: u64,
    pub filter_category: Vec<String>,
    pub filter_tags: Vec<String>,
    pub blacklist_infohash: Vec<String>,
    pub delete_files: bool,
    pub enable: bool,
}

impl Default for CleanRule {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "默认规则".to_string(),
            min_ratio: 2.0,
            min_seed_seconds: 86400,
            min_free_disk_bytes: 20 * 1024 * 1024 * 1024,
            filter_category: vec![],
            filter_tags: vec![],
            blacklist_infohash: vec![],
            delete_files: false,
            enable: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanLog {
    pub time: i64,
    pub dry_run: bool,
    pub total_torrents: usize,
    pub to_remove: usize,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewTorrent {
    pub infohash: String,
    pub name: String,
    pub ratio: f64,
    pub seed_time: u64,
    pub category: String,
    pub tags: Vec<String>,
    pub will_remove: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub task_id: u64,
    pub dry_run: bool,
    pub list: Vec<PreviewTorrent>,
    pub log: ScanLog,
}