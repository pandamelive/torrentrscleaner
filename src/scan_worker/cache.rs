use std::time::{Instant, Duration};
use crate::models::PreviewTorrent;

#[derive(Debug)]
pub struct TorrentCache {
    data: Option<Vec<PreviewTorrent>>,
    stamp: Instant,
    ttl: Duration,
}

impl TorrentCache {
    pub fn new(ttl_sec:u64) -> Self {
        Self {
            data: None,
            stamp: Instant::now(),
            ttl: Duration::from_secs(ttl_sec),
        }
    }
    pub fn get(&self) -> Option<&Vec<PreviewTorrent>> {
        if self.stamp.elapsed() < self.ttl {
            self.data.as_ref()
        } else {
            None
        }
    }
    pub fn set(&mut self, val:Vec<PreviewTorrent>) {
        self.data = Some(val);
        self.stamp = Instant::now();
    }
    pub fn invalidate(&mut self) {
        self.data = None;
    }
}