pub mod amqp;
pub mod error;
pub mod retry;

use crate::error::Result;

use std::time::Duration;

use async_trait::async_trait;

pub type BoxedTag = Box<dyn ProcessTag>;

#[async_trait]
pub trait SendQueue<T> {
    async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<()>;
}

#[async_trait]
pub trait ReceiveQueue<T> {
    async fn dequeue(&self) -> Result<Option<(T, BoxedTag)>>;
}

#[async_trait]
pub trait ProcessTag: Send + Sync + 'static {
    async fn resolve(self) -> Result<()>;
    async fn reject(self) -> Result<()>;
}
