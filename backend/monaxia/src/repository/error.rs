use std::result::Result as StdResult;

use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("unsupported feature: {0}")]
    Unsupported(&'static str),

    #[error("database error: {0}")]
    Database(#[from] SqlxError),

    #[error("other error: {0}")]
    Other(String),
}
