use thiserror::Error;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("db database error:{0}")]
    Database(#[from] redb::DatabaseError),
    #[error("db transaction error:{0}")]
    Transaction(#[from] redb::TransactionError),
    #[error("db table error:{0}")]
    Table(#[from] redb::TableError),
    #[error("db commit error:{0}")]
    Commit(#[from] redb::CommitError),
    #[error("json serde error:{0}")]
    Json(#[from] serde_json::Error),
    #[error("downloader client error:{0}")]
    Downloader(#[from] anyhow::Error),
    #[error("task busy: another scan running")]
    TaskBusy,
    #[error("invalid config:{0}")]
    ConfigInvalid(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = match &self {
            AppError::ConfigInvalid(_) => StatusCode::BAD_REQUEST,
            AppError::TaskBusy => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, self.to_string()).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
