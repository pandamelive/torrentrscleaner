use anyhow::Result;
use async_trait::async_trait;
use crate::models::PreviewTorrent;

#[async_trait]
pub trait DownloaderClient: Send + Sync {
    async fn fetch_all_preview(&self) -> Result<Vec<PreviewTorrent>>;
    async fn remove_torrent(&self, infohash: &str, delete_files: bool) -> Result<()>;
    async fn test_connect(&self) -> Result<()>;
}