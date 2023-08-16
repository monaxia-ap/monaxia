use super::{
    AMQP_PERSISTENT_DELIVERY_MODE, AMQP_X_DELAY, AMQP_X_DELAYED_MESSAGE, AMQP_X_DELAYED_TYPE,
    DEFAULT_EXCHANGE_NAME, DELAYED_EXCHANGE_NAME,
};
use crate::{
    error::{Error, Result},
    queue::SendQueue,
};

use std::{fmt::Debug, marker::PhantomData, time::Duration};

use async_trait::async_trait;
use lapin::{
    options::{
        BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions, QueueDeclareOptions,
    },
    types::{AMQPValue, FieldTable},
    BasicProperties, Channel, ExchangeKind,
};
use serde::Serialize;
use tracing::instrument;

/// Sender queue that uses AMQP client.
#[derive(Debug)]
pub struct SenderQueue<T> {
    _payload_type: PhantomData<fn() -> T>,
    channel: Channel,
    worker_name: String,
    queue_name: String,
}

impl<T> SenderQueue<T>
where
    T: Debug + Serialize + Send + Sync + 'static,
{
    pub async fn new(
        channel: Channel,
        queue_name: impl Into<String>,
        worker_name: impl Into<String>,
    ) -> Result<SenderQueue<T>> {
        let queue_name = queue_name.into();
        let worker_name = worker_name.into();
        initialize_channel(&channel, &queue_name).await?;
        Ok(SenderQueue {
            channel,
            worker_name,
            queue_name,
            _payload_type: Default::default(),
        })
    }

    #[instrument(skip(self), fields(tag = format!("{} on {}", self.worker_name, self.queue_name)))]
    async fn publish(&self, data: T) -> Result<()> {
        // job is persistent
        let props = BasicProperties::default().with_delivery_mode(AMQP_PERSISTENT_DELIVERY_MODE);
        let payload = rmp_serde::to_vec_named(&data).map_err(|e| Error::Serialization(e.into()))?;

        let confirm = self
            .channel
            .basic_publish(
                DEFAULT_EXCHANGE_NAME,
                &self.queue_name,
                BasicPublishOptions::default(),
                &payload,
                props,
            )
            .await
            .map_err(|e| Error::Queue(e.into()))?;
        confirm.await.map_err(|e| Error::Delivery(e.into()))?;

        Ok(())
    }

    #[instrument(skip(self), fields(tag = format!("{} on {}", self.worker_name, self.queue_name)))]
    async fn publish_delayed(&self, data: T, delay: Duration) -> Result<()> {
        // job is delayed and persistent
        let headers = {
            let delay_ms = delay.as_millis() as i64;
            let mut ft = FieldTable::default();
            ft.insert(AMQP_X_DELAY.into(), AMQPValue::LongLongInt(delay_ms));
            ft
        };
        let props = BasicProperties::default()
            .with_delivery_mode(AMQP_PERSISTENT_DELIVERY_MODE)
            .with_headers(headers);
        let payload = rmp_serde::to_vec_named(&data).map_err(|e| Error::Serialization(e.into()))?;

        let confirm = self
            .channel
            .basic_publish(
                DELAYED_EXCHANGE_NAME,
                &self.queue_name,
                BasicPublishOptions::default(),
                &payload,
                props,
            )
            .await
            .map_err(|e| Error::Queue(e.into()))?;
        confirm.await.map_err(|e| Error::Delivery(e.into()))?;

        Ok(())
    }
}

#[async_trait]
impl<T> SendQueue<T> for SenderQueue<T>
where
    T: Debug + Serialize + Send + Sync + 'static,
{
    async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<()> {
        if let Some(delay) = delay {
            self.publish_delayed(data, delay).await?;
        } else {
            self.publish(data).await?;
        }
        Ok(())
    }
}

async fn initialize_channel(channel: &Channel, queue_name: &str) -> Result<()> {
    // declare durable exchange for delayed messages
    let arguments = {
        let mut ft = FieldTable::default();
        // should be `ExchangeKind::Direct.kind()`,
        // but kind() is not accessible.
        ft.insert(
            AMQP_X_DELAYED_TYPE.into(),
            AMQPValue::LongString("direct".into()),
        );
        ft
    };
    channel
        .exchange_declare(
            DELAYED_EXCHANGE_NAME,
            ExchangeKind::Custom(AMQP_X_DELAYED_MESSAGE.into()),
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                ..Default::default()
            },
            arguments,
        )
        .await
        .map_err(|e| Error::Queue(e.into()))?;

    // declare queue
    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                auto_delete: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .map_err(|e| Error::Queue(e.into()))?;

    // enable message confirmation
    channel
        .confirm_select(ConfirmSelectOptions::default())
        .await
        .map_err(|e| Error::Queue(e.into()))?;
    Ok(())
}
