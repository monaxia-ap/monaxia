mod ap;
mod root;
mod user;

use crate::{constant::create_http_client, repository_impl::construct_container_db};

use std::sync::Arc;

use anyhow::Result;
use lapin::{Connection as LapinConnection, ConnectionProperties};
use monaxia_data::config::Config;
use monaxia_job::job::MxJob;
use monaxia_queue::{
    job::{Consumer, Producer},
    queue::amqp::{create_amqp_consumer, create_amqp_producer},
};
use monaxia_repository::Container;
use reqwest::Client;
use tokio::spawn;
use tracing::info;

pub struct WorkerState {
    name: String,
    consumer: Consumer<MxJob>,
    job_state: JobState,
}

#[derive(Clone)]
struct JobState {
    pub config: Arc<Config>,
    pub container: Container,
    pub http_client: Client,
}

pub async fn spawn_workers(config: Arc<Config>, consumers: Vec<Consumer<MxJob>>) -> Result<()> {
    let container = construct_container_db(&config).await?;
    let http_client = create_http_client()?;

    info!("spawning {} workers", consumers.len());
    for (i, consumer) in consumers.into_iter().enumerate() {
        spawn(root::worker(WorkerState {
            name: format!("worker-{i}"),
            consumer,
            job_state: JobState {
                config: config.clone(),
                container: container.clone(),
                http_client: http_client.clone(),
            },
        }));
    }

    Ok(())
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
