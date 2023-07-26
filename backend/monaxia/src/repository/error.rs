use std::result::Result as StdResult;

use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;
use tokio::io::Error as TokioIoError;

pub type Result<T> = StdResult<T, Error>;

#[allow(dead_code)]
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("unsupported feature: {0}")]
    Unsupported(String),

    #[error("database error: {0}")]
    Database(#[from] SqlxError),

    #[error("IO error: {0}")]
    Io(#[from] TokioIoError),

    #[error("other error: {0}")]
    Other(String),
}
