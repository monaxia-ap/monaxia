use crate::{
    error::{Error, Result},
    queue::{BoxedTag, ProcessTag, ReceiveQueue},
};

use std::{fmt::Debug, marker::PhantomData};

use async_trait::async_trait;
use futures::{lock::Mutex, StreamExt};
use lapin::{
    acker::Acker,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
    Channel, Consumer,
};
use serde::de::DeserializeOwned;
use tracing::instrument;

/// Sender queue that uses AMQP client.
#[derive(Debug)]
pub struct ReceiverQueue<T> {
    _payload_type: PhantomData<fn() -> T>,
    _channel: Channel,
    consumer: Mutex<Consumer>,
    worker_name: String,
    queue_name: String,
}

impl<T> ReceiverQueue<T>
where
    T: Debug + DeserializeOwned + Send + Sync + 'static,
{
    pub async fn new(
        channel: Channel,
        queue_name: impl Into<String>,
        worker_name: impl Into<String>,
    ) -> Result<ReceiverQueue<T>> {
        let queue_name = queue_name.into();
        let worker_name = worker_name.into();
        let consumer = channel
            .basic_consume(
                &queue_name,
                &worker_name,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| Error::Queue(e.into()))?;

        Ok(ReceiverQueue {
            _channel: channel,
            consumer: Mutex::new(consumer),
            worker_name,
            queue_name,
            _payload_type: Default::default(),
        })
    }

    #[instrument(skip(self), fields(tag = format!("{} on {}", self.worker_name, self.queue_name)))]
    async fn consume_one(&self) -> Result<Option<(T, BoxedTag)>> {
        let delivery = {
            let mut locked = self.consumer.lock().await;
            match locked.next().await {
                Some(d) => d.map_err(|e| Error::Queue(e.into()))?,
                None => return Ok(None),
            }
        };

        let tag = Box::new(Tag(delivery.acker));
        match rmp_serde::from_slice(&delivery.data) {
            Ok(payload) => Ok(Some((payload, tag))),
            Err(e) => Err(Error::Deserialization(e.into(), tag)),
        }
    }
}

#[async_trait]
impl<T> ReceiveQueue<T> for ReceiverQueue<T>
where
    T: Debug + DeserializeOwned + Send + Sync + 'static,
{
    async fn dequeue(&self) -> Result<Option<(T, BoxedTag)>> {
        let Some((payload, tag)) = self.consume_one().await? else {
            return Ok(None);
        };
        Ok(Some((payload, tag)))
    }
}

#[derive(Debug)]
pub struct Tag(Acker);

#[async_trait]
impl ProcessTag for Tag {
    async fn resolve(self: Box<Self>) -> Result<()> {
        self.0
            .ack(BasicAckOptions::default())
            .await
            .map_err(|e| Error::Delivery(e.into()))?;
        Ok(())
    }

    async fn reject(self: Box<Self>) -> Result<()> {
        self.0
            .nack(BasicNackOptions {
                requeue: false,
                ..Default::default()
            })
            .await
            .map_err(|e| Error::Delivery(e.into()))?;
        Ok(())
    }
}
