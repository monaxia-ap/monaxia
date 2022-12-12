pub mod sink;

use std::error::Error as StdError;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

/// Abstraction layer for job result tracer interface.
#[async_trait]
pub trait Tracer {
    /// Payload data type.
    type Data: Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Error type from backend.
    type Error: StdError + Send + Sync + 'static;

    /// Pushes trace data.
    async fn push(&self, data: Self::Data) -> Result<(), Self::Error>;
}
