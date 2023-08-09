pub mod amqp;
pub mod memory;

use crate::error::Result;

use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;

pub type BoxedTag = Box<dyn ProcessTag>;

#[async_trait]
pub trait SendQueue<T>: Debug + Send + Sync + 'static {
    async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<()>;
}

#[async_trait]
pub trait ReceiveQueue<T>: Debug + Send + Sync + 'static {
    async fn dequeue(&self) -> Result<Option<(T, BoxedTag)>>;
}

#[async_trait]
pub trait ProcessTag: Send + Sync + 'static {
    async fn resolve(self) -> Result<()>;
    async fn reject(self) -> Result<()>;
}
