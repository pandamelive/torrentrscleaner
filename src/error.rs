use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("db error:{0}")]
    Db(#[from] redb::Error),
    #[error("json serde error:{0}")]
    Json(#[from] serde_json::Error),
    #[error("downloader client error:{0}")]
    Downloader(#[from] anyhow::Error),
    #[error("task busy: another scan running")]
    TaskBusy,
    #[error("invalid config:{0}")]
    ConfigInvalid(String),
}

pub type AppResult<T> = Result<T, AppError>;