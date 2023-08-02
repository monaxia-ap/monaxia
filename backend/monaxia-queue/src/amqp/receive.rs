use super::{Error, Result};
use crate::ReceiveQueue;

use std::marker::PhantomData;

use async_trait::async_trait;
use lapin::{acker::Acker, Channel};
use serde::{de::DeserializeOwned, Serialize};

/// Sender queue that uses AMQP client.
pub struct ReceiverQueue<T> {
    channel: Channel,
    worker_name: String,
    queue_name: String,
    _payload_type: PhantomData<fn() -> T>,
}

impl<T> ReceiverQueue<T> where T: DeserializeOwned + Send + Sync + 'static {}

#[async_trait]
impl<T> ReceiveQueue<T> for ReceiverQueue<T>
where
    T: Serialize + Send + Sync + 'static,
{
    type Error = Error;
    type Tag = Acker;

    async fn dequeue(&self) -> Result<(T, Self::Tag)> {
        todo!();
    }

    async fn resolve(&self, tag: Self::Tag) -> Result<()> {
        todo!();
    }

    async fn reject(&self, tag: Self::Tag) -> Result<()> {
        todo!();
    }
}
