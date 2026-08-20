use anyhow::Result;
use async_trait::async_trait;
use qbit_rs::QbitClient;
use crate::downloader::traits::DownloaderClient;
use crate::models::PreviewTorrent;

pub struct QbDownloader {
    client: QbitClient,
}

impl QbDownloader {
    pub async fn new(addr: &str, user: &str, pass: &str) -> Result<Self> {
        let cli = QbitClient::new(addr.to_string(), user.to_string(), pass.to_string()).await?;
        Ok(Self { client: cli })
    }
}

#[async_trait]
impl DownloaderClient for QbDownloader {
    async fn fetch_all_preview(&self) -> Result<Vec<PreviewTorrent>> {
        let list = self.client.torrent_list(None).await?;
        let mut res = Vec::new();
        for t in list {
            res.push(PreviewTorrent {
                infohash: t.hash.clone(),
                name: t.name,
                ratio: t.ratio.unwrap_or(0.0),
                seed_time: t.seeding_time.unwrap_or(0) as u64,
                category: t.category.unwrap_or_default(),
                tags: t.tags.map(|x| x.split(',').map(|s| s.to_string()).collect()).unwrap_or_default(),
                will_remove: false,
                reason: String::new(),
            });
        }
        Ok(res)
    }

    async fn remove_torrent(&self, hash: &str, delete_files: bool) -> Result<()> {
        self.client.torrent_delete(hash, delete_files).await?;
        Ok(())
    }

    async fn test_connect(&self) -> Result<()> {
        self.client.version().await?;
        Ok(())
    }
}