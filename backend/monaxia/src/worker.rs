mod root;

use anyhow::Result;
use lapin::{Connection as LapinConnection, ConnectionProperties};
use monaxia_data::config::Config;
use monaxia_job::job::MxJob;
use monaxia_queue::{
    job::{Consumer, Producer},
    queue::amqp::{create_amqp_consumer, create_amqp_producer},
};
use tokio::spawn;
use tracing::info;

pub async fn spawn_workers(consumers: Vec<Consumer<MxJob>>) {
    info!("spawning {} workers", consumers.len());

    for (i, consumer) in consumers.into_iter().enumerate() {
        spawn(root::worker(i, consumer));
    }
}

pub async fn create_queues(config: &Config) -> Result<(Producer<MxJob>, Vec<Consumer<MxJob>>)> {
    let conn = LapinConnection::connect(&config.queue.url, ConnectionProperties::default()).await?;
    let producer = create_amqp_producer(&conn, "producer").await?;
    let consumers = create_amqp_consumer(&conn, "consumer", config.queue.workers).await?;

    Ok((producer, consumers))
}

#[cfg(test)]
pub fn create_test_queues() -> (Producer<MxJob>, Consumer<MxJob>) {
    use futures::channel::mpsc::channel;
    use monaxia_queue::queue::memory::{create_memory_consumer, create_memory_producer};

    let (sender, receiver) = channel(8);
    let producer = create_memory_producer(sender.clone());
    let consumer = create_memory_consumer(sender, receiver);
    (producer, consumer)
}
