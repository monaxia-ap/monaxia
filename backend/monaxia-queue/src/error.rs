use std::{error::Error as StdError, result::Result as StdResult};

use thiserror::Error as ThisError;

use crate::queue::BoxedTag;

pub type Result<T> = StdResult<T, Error>;

type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// Wrapped error type for AMQP queue.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error from AMQP.
    #[error("queue error: {0}")]
    Queue(BoxError),

    /// Error from serialization.
    #[error("serialization error: {0}")]
    Serialization(BoxError),

    #[error("deserialization error: {0}")]
    Deserialization(BoxError, BoxedTag),

    /// Error from tag operation.
    #[error("delivery error: {0}")]
    Delivery(BoxError),
}
