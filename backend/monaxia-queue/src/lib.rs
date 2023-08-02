pub mod amqp;
pub mod retry;

use std::{error::Error as StdError, time::Duration};

use async_trait::async_trait;

#[async_trait]
pub trait SendQueue<T> {
    type Error: Send + Sync + StdError + 'static;

    async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait ReceiveQueue<T> {
    type Error: Send + Sync + StdError + 'static;
    type Tag: Send + Sync + 'static;

    async fn dequeue(&self) -> Result<(T, Self::Tag), Self::Error>;
    async fn resolve(&self, tag: Self::Tag) -> Result<(), Self::Error>;
    async fn reject(&self, tag: Self::Tag) -> Result<(), Self::Error>;
}
