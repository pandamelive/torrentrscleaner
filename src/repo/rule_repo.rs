use super::AppRepo;
use crate::models::CleanRule;
use crate::error::AppResult;
use redb::ReadableTable;
use serde_json;
use super::TABLE_RULES;

impl AppRepo {
    pub fn save_rules(&self, rules:&[CleanRule]) -> AppResult<()> {
        let json = serde_json::to_string(rules)?;
        let w = self.db().begin_write()?;
        let mut t = w.open_table(TABLE_RULES)?;
        t.insert("list", json.as_str())?;
        w.commit()?;
        Ok(())
    }
    pub fn load_rules(&self) -> AppResult<Vec<CleanRule>> {
        let r = self.db().begin_read()?;
        let t = r.open_table(TABLE_RULES)?;
        let v = t.get("list")?;
        match v {
            Some(v) => Ok(serde_json::from_str(v.value())?),
            None => Ok(vec![CleanRule::default()]),
        }
    }
}