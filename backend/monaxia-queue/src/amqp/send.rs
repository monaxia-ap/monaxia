use super::{Error, Result};
use crate::SendQueue;

use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use lapin::Channel;
use serde::Serialize;

/// Sender queue that uses AMQP client.
pub struct SenderQueue<T> {
    channel: Channel,
    worker_name: String,
    queue_name: String,
    _payload_type: PhantomData<fn() -> T>,
}

impl<T> SenderQueue<T> where T: Serialize + Send + Sync + 'static {}

#[async_trait]
impl<T> SendQueue<T> for SenderQueue<T>
where
    T: Serialize + Send + Sync + 'static,
{
    type Error = Error;

    async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<()> {
        todo!()
    }
}
