use super::AppRepo;
use crate::models::AppConfig;
use crate::error::AppResult;
use redb::ReadableTable;
use serde_json;
use super::TABLE_CONFIG;

impl AppRepo {
    pub fn save_config(&self, cfg:&AppConfig) -> AppResult<()> {
        let json = serde_json::to_string(cfg)?;
        let w = self.db().begin_write()?;
        {
            let mut t = w.open_table(TABLE_CONFIG)?;
            t.insert("main", json.as_str())?;
        }
        w.commit()?;
        Ok(())
    }
    pub fn load_config(&self) -> AppResult<AppConfig> {
        let r = self.db().begin_read()?;
        let t = r.open_table(TABLE_CONFIG)?;
        let v = t.get("main")?;
        match v {
            Some(v) => Ok(serde_json::from_str(v.value())?),
            None => Ok(AppConfig::default()),
        }
    }
}
