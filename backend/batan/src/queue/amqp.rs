use crate::queue::{QueueReceive, QueueSend};

use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use bincode::{
    deserialize as bincode_deserialize, serialize as bincode_serialize, Error as BincodeError,
};
use futures::StreamExt;
use lapin::{
    acker::Acker,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions,
        ConfirmSelectOptions, ExchangeDeclareOptions,
    },
    types::{AMQPValue, FieldTable},
    BasicProperties, Channel, Consumer, Error as LapinError, ExchangeKind,
};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error as ThisError;
use tracing::instrument;

const AMQP_PERSISTENT_DELIVERY_MODE: u8 = 2;
const AMQP_X_DELAY: &str = "x-delay";
const AMQP_X_DELAYED_TYPE: &str = "x-delayed-type";
const AMQP_X_DELAYED_MESSAGE: &str = "x-delayed-message";
const DEFAULT_EXCHANGE_NAME: &str = "";
const DELAYED_EXCHANGE_NAME: &str = "batan-delayed-exchange";

/// Wrapped error type for AMQP queue.
#[derive(Debug, ThisError)]
pub enum AmqpQueueError {
    /// Error from AMQP.
    #[error("AMQP error")]
    Amqp(#[from] LapinError),

    /// Error from serialization.
    #[error("serialization error")]
    Serialization(#[from] BincodeError),
}

/// Sender queue that uses AMQP client.
pub struct AmqpSenderQueue<T> {
    channel: Channel,
    worker_name: String,
    queue_name: String,
    _payload_type: PhantomData<fn() -> T>,
}

impl<T> AmqpSenderQueue<T>
where
    T: Serialize + Send + Sync + 'static,
{
    /// Establishes new AMQP publisher and uses it.
    pub async fn new(
        channel: Channel,
        queue_name: impl Into<String>,
        worker_name: impl Into<String>,
    ) -> Result<AmqpSenderQueue<T>, LapinError> {
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;
        Self::declare_delayed_exchange(&channel).await?;

        Ok(AmqpSenderQueue {
            channel,
            worker_name: worker_name.into(),
            queue_name: queue_name.into(),
            _payload_type: Default::default(),
        })
    }

    /// Declares delayed exchange explicitly.
    async fn declare_delayed_exchange(channel: &Channel) -> Result<(), LapinError> {
        let arguments = {
            let mut ft = FieldTable::default();
            // ExchangeKind::kind is pub(crate)
            ft.insert(
                AMQP_X_DELAYED_TYPE.into(),
                AMQPValue::ShortString("direct".into()),
            );
            ft
        };
        let options = ExchangeDeclareOptions {
            durable: true,
            auto_delete: false,
            ..Default::default()
        };

        channel
            .exchange_declare(
                DELAYED_EXCHANGE_NAME,
                ExchangeKind::Custom(AMQP_X_DELAYED_MESSAGE.into()),
                options,
                arguments,
            )
            .await?;
        Ok(())
    }

    /// Publishes a payload immediately.
    #[instrument(skip(self, data), fields(tag = format!("{} on {}", self.worker_name, self.queue_name)))]
    async fn publish(&mut self, data: T) -> Result<(), AmqpQueueError> {
        let props = BasicProperties::default().with_delivery_mode(AMQP_PERSISTENT_DELIVERY_MODE);
        let payload = bincode_serialize(&data)?;

        let confirm = self
            .channel
            .basic_publish(
                DEFAULT_EXCHANGE_NAME,
                &self.queue_name,
                BasicPublishOptions::default(),
                &payload,
                props,
            )
            .await?;
        let _confirmed = confirm.await?;

        Ok(())
    }

    /// Publishes a payload with delay.
    #[instrument(skip(self, data, delay), fields(tag = format!("{} on {}", self.worker_name, self.queue_name)))]
    async fn publish_delayed(&mut self, data: T, delay: Duration) -> Result<(), AmqpQueueError> {
        let headers = {
            let delay_ms = delay.as_millis() as i64;
            let mut ft = FieldTable::default();
            ft.insert(AMQP_X_DELAY.into(), AMQPValue::LongLongInt(delay_ms));
            ft
        };
        let props = BasicProperties::default()
            .with_delivery_mode(AMQP_PERSISTENT_DELIVERY_MODE)
            .with_headers(headers);
        let payload = bincode::serialize(&data)?;

        let confirm = self
            .channel
            .basic_publish(
                DELAYED_EXCHANGE_NAME,
                &self.queue_name,
                BasicPublishOptions::default(),
                &payload,
                props,
            )
            .await?;
        let _confirmed = confirm.await?;

        todo!();
    }
}

#[async_trait]
impl<T> QueueSend for AmqpSenderQueue<T>
where
    T: Serialize + Send + Sync + 'static,
{
    type Data = T;
    type Error = AmqpQueueError;

    async fn enqueue(&mut self, data: T, delay: Option<Duration>) -> Result<(), AmqpQueueError> {
        if let Some(delay) = delay {
            self.publish_delayed(data, delay).await?;
        } else {
            self.publish(data).await?;
        }
        Ok(())
    }
}

/// Sender queue that uses AMQP client.
pub struct AmqpReceiverQueue<T> {
    _channel: Channel,
    consumer: Consumer,
    worker_name: String,
    queue_name: String,
    _payload_type: PhantomData<fn() -> T>,
}

impl<T> AmqpReceiverQueue<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    /// Establishes new AMQP consumer and uses it.
    pub async fn new(
        channel: Channel,
        queue_name: impl Into<String>,
        worker_name: impl Into<String>,
    ) -> Result<AmqpReceiverQueue<T>, LapinError> {
        let queue_name = queue_name.into();
        let worker_name = worker_name.into();
        let consumer = channel
            .basic_consume(
                &queue_name,
                &worker_name,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(AmqpReceiverQueue {
            _channel: channel,
            consumer,
            worker_name,
            queue_name,
            _payload_type: Default::default(),
        })
    }

    /// Consumes one data.
    #[instrument(skip(self), fields(tag = format!("{} on {}", self.worker_name, self.queue_name)))]
    async fn consume_one(&mut self) -> Result<Option<(T, Acker)>, AmqpQueueError> {
        let delivery = match self.consumer.next().await {
            Some(d) => d?,
            None => return Ok(None),
        };

        let payload = bincode_deserialize(&delivery.data)?;
        Ok(Some((payload, delivery.acker)))
    }
}

#[async_trait]
impl<T> QueueReceive for AmqpReceiverQueue<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    type Data = T;
    type Tag = Acker;
    type Error = AmqpQueueError;

    async fn dequeue(&mut self) -> Result<Option<(T, Acker)>, AmqpQueueError> {
        let next = self.consume_one().await?;
        Ok(next)
    }

    async fn resolve(&mut self, tag: Acker) -> Result<(), AmqpQueueError> {
        tag.ack(BasicAckOptions::default()).await?;
        Ok(())
    }

    async fn reject(&mut self, tag: Acker) -> Result<(), AmqpQueueError> {
        tag.nack(BasicNackOptions {
            requeue: false,
            ..Default::default()
        })
        .await?;
        Ok(())
    }
}
