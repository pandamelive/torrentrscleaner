use super::AppRepo;
use crate::models::ScanLog;
use crate::error::AppResult;
use redb::ReadableTable;
use serde_json;
use super::TABLE_LOGS;

impl AppRepo {
    pub fn save_log(&self, seq: u64, log: &ScanLog) -> AppResult<()> {
        let json = serde_json::to_string(log)?;
        let w = self.db().begin_write()?;
        {
            let mut t = w.open_table(TABLE_LOGS)?;
            t.insert(seq, json.as_str())?;
        }
        w.commit()?;
        Ok(())
    }

    pub fn load_logs(&self) -> AppResult<Vec<ScanLog>> {
        let r = self.db().begin_read()?;
        let t = r.open_table(TABLE_LOGS)?;
        let mut res = Vec::new();
        let iter = t.iter()?;
        for kv in iter {
            let (_k, v) = kv?;
            res.push(serde_json::from_str(v)?);
        }
        Ok(res)
    }
}
