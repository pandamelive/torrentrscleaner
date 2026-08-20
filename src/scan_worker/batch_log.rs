use crate::repo::AppRepo;
use crate::models::ScanLog;
use crate::error::AppResult;
use std::collections::VecDeque;

pub struct BatchLogWriter {
    queue: VecDeque<(u64, ScanLog)>,
    repo: AppRepo,
    seq: u64,
}

impl BatchLogWriter {
    pub fn new(repo:AppRepo) -> Self {
        Self { queue:VecDeque::new(), repo, seq:0 }
    }
    pub fn push(&mut self, log:ScanLog) {
        self.queue.push_back((self.seq, log));
        self.seq +=1;
    }
    pub fn flush(&mut self) -> AppResult<()> {
        while let Some((sid,log)) = self.queue.pop_front() {
            self.repo.append_scan_log(sid, &log)?;
        }
        Ok(())
    }
}