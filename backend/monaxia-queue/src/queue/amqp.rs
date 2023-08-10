mod receive;
mod send;

pub use self::{receive::ReceiverQueue, send::SenderQueue};

use crate::{
    error::{Error, Result},
    job::{Consumer, Producer},
};

use std::{fmt::Debug, sync::Arc};

use lapin::Connection;
use serde::{de::DeserializeOwned, Serialize};
use tracing::debug;

const AMQP_PERSISTENT_DELIVERY_MODE: u8 = 2;
const AMQP_X_DELAY: &str = "x-delay";
const AMQP_X_DELAYED_TYPE: &str = "x-delayed-type";
const AMQP_X_DELAYED_MESSAGE: &str = "x-delayed-message";
const DEFAULT_EXCHANGE_NAME: &str = "";
const DELAYED_EXCHANGE_NAME: &str = "monaxia-delayed-exchange";

const QUEUE_NAME_BASE: &str = "mx-queue";
const WORKER_NAME_BASE: &str = "mx-worker";

pub async fn create_amqp_producer<T>(conn: &Connection, worker_suffix: &str) -> Result<Producer<T>>
where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    let sender_queue = create_amqp_sender_queue(conn, worker_suffix).await?;
    Ok(Producer {
        sender: Arc::new(sender_queue),
    })
}

pub async fn create_amqp_consumer<T>(
    conn: &Connection,
    worker_suffix: &str,
    count: usize,
) -> Result<Vec<Consumer<T>>>
where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    let shared_sender = Arc::new(create_amqp_sender_queue(conn, worker_suffix).await?);

    let mut consumers = vec![];
    for i in 1..=count {
        let receiver_queue =
            create_amqp_receiver_queue(conn, &format!("{worker_suffix}-{i}")).await?;
        consumers.push(Consumer {
            receiver: Box::new(receiver_queue),
            shared_sender: shared_sender.clone(),
        })
    }
    Ok(consumers)
}

async fn create_amqp_sender_queue<T>(
    conn: &Connection,
    worker_suffix: &str,
) -> Result<SenderQueue<T>>
where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    let channel = conn
        .create_channel()
        .await
        .map_err(|e| Error::Queue(e.into()))?;
    debug!("AMQP channel for sender queue created");
    let worker_name = format!("{}-{worker_suffix}", WORKER_NAME_BASE);
    let sender = SenderQueue::new(channel, QUEUE_NAME_BASE, worker_name).await?;

    Ok(sender)
}

async fn create_amqp_receiver_queue<T: Debug>(
    conn: &Connection,
    worker_suffix: &str,
) -> Result<ReceiverQueue<T>>
where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    let channel = conn
        .create_channel()
        .await
        .map_err(|e| Error::Queue(e.into()))?;
    debug!("AMQP channel for receiver queue created");
    let worker_name = format!("{}-{worker_suffix}", WORKER_NAME_BASE);
    let receiver = ReceiverQueue::new(channel, QUEUE_NAME_BASE, worker_name).await?;

    Ok(receiver)
}
