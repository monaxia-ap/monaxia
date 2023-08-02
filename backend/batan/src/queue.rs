pub mod amqp;

use std::{error::Error as StdError, time::Duration};

use async_trait::async_trait;

/// Abstraction layer for queue sending interface.
#[async_trait]
pub trait QueueSend<T> {
    /// Error type from backend.
    type Error: StdError + Send + Sync + 'static;

    /// Enqueues job payload with specified delay.
    async fn enqueue(&mut self, data: T, delay: Option<Duration>) -> Result<(), Self::Error>;
}

/// Abstraction layer for queue receiving interface.
#[async_trait]
pub trait QueueReceive<T> {
    /// Payload acknowledge tag type.
    type Tag: Send + Sync + 'static;

    /// Error type from backend.
    type Error: StdError + Send + Sync + 'static;

    /// Enqueues job payload with specified delay.
    async fn dequeue(&mut self) -> Result<Option<(T, Self::Tag)>, Self::Error>;

    /// Marks a job as resolved.
    async fn resolve(&mut self, tag: Self::Tag) -> Result<(), Self::Error>;

    /// Marks a job as rejected.
    async fn reject(&mut self, tag: Self::Tag) -> Result<(), Self::Error>;
}
