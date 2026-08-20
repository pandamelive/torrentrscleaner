pub mod config_repo;
pub mod rule_repo;
pub mod log_repo;

use redb::{Database, TableDefinition};
use std::path::Path;
use crate::error::AppResult;

const TABLE_CONFIG: TableDefinition<&str, &str> = TableDefinition::new("config");
const TABLE_RULES: TableDefinition<&str, &str> = TableDefinition::new("rules");
const TABLE_LOGS: TableDefinition<u64, &str> = TableDefinition::new("scan_logs");

#[derive(Debug, Clone)]
pub struct AppRepo {
    db: &'static Database,
}

impl AppRepo {
    pub fn open(path: &Path) -> AppResult<Self> {
        let db = Box::leak(Box::new(Database::create(path)?));
        let w = db.begin_write()?;
        w.open_table(TABLE_CONFIG)?;
        w.open_table(TABLE_RULES)?;
        w.open_table(TABLE_LOGS)?;
        w.commit()?;
        Ok(Self { db })
    }
    pub fn db(&self) -> &'static Database { self.db }
}