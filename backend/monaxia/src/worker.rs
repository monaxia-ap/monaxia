mod ap;
mod user;

use crate::{misc::create_http_client, repository_impl::construct_container_db};

use std::sync::Arc;

use anyhow::Result;
use lapin::{Connection as LapinConnection, ConnectionProperties};
use monaxia_data::config::Config;
use monaxia_job::job::{Job, MxJob};
use monaxia_queue::{
    job::{Consumer, Producer},
    queue::amqp::{create_amqp_consumer, create_amqp_producer},
};
use monaxia_repository::Container;
use reqwest::Client;
use serde_variant::to_variant_name;
use tokio::spawn;
use tracing::{error, info, instrument};

pub struct WorkerState {
    name: String,
    consumer: Consumer<MxJob>,
    job_state: JobState,
}

#[derive(Clone)]
struct JobState {
    pub config: Arc<Config>,
    pub container: Container,
    pub producer: Producer<MxJob>,
    pub http_client: Client,
}

pub async fn start_workers(config: Arc<Config>) -> Result<Producer<MxJob>> {
    let conn = LapinConnection::connect(&config.queue.url, ConnectionProperties::default()).await?;
    let producer = create_amqp_producer(&conn, "producer").await?;
    let consumers = create_amqp_consumer(&conn, "consumer", config.queue.workers).await?;

    spawn_workers(config.clone(), producer.clone(), consumers).await?;

    Ok(producer)
}

#[cfg(test)]
pub fn start_test_workers() -> (Producer<MxJob>, Consumer<MxJob>) {
    use futures::channel::mpsc::channel;
    use monaxia_queue::queue::memory::{create_memory_consumer, create_memory_producer};

    let (sender, receiver) = channel(8);
    let producer = create_memory_producer(sender.clone());
    let consumer = create_memory_consumer(sender, receiver);
    (producer, consumer)
}

async fn spawn_workers(
    config: Arc<Config>,
    producer: Producer<MxJob>,
    consumers: Vec<Consumer<MxJob>>,
) -> Result<()> {
    let container = construct_container_db(&config).await?;
    let http_client = create_http_client(&config)?;

    info!("spawning {} workers", consumers.len());
    for (i, consumer) in consumers.into_iter().enumerate() {
        spawn(worker(WorkerState {
            name: format!("worker-{i}"),
            consumer,
            job_state: JobState {
                config: config.clone(),
                container: container.clone(),
                producer: producer.clone(),
                http_client: http_client.clone(),
            },
        }));
    }

    Ok(())
}

#[instrument(skip(state), fields(worker = state.name))]
async fn worker(state: WorkerState) {
    loop {
        let (job, delivery) = match state.consumer.fetch().await {
            Ok(Some((j, d))) => (j, d),
            Err(e) => {
                error!("queue fetch error: {e}");
                continue;
            }
            Ok(None) => {
                info!("Queue closed");
                return;
            }
        };

        let job_state = state.job_state.clone();
        let job_payload = job.job().clone();
        let job_tag = job.tag().to_string();

        match do_job(job_state, job_payload, job_tag).await {
            Ok(()) => match state.consumer.mark_success(delivery).await {
                Ok(()) => (),
                Err(e) => {
                    error!("queue ack error: {e}");
                    continue;
                }
            },
            Err(e) => {
                error!("job error: {e}");
                match state.consumer.mark_failure(delivery).await {
                    Ok(()) => (),
                    Err(e) => {
                        error!("queue nack error: {e}");
                        continue;
                    }
                }
                let Some((data, delay)) = job.next() else {
                    continue;
                };
                match state.consumer.enqueue(data, Some(delay)).await {
                    Ok(()) => (),
                    Err(e) => {
                        error!("retry enqueue error: {e}");
                        continue;
                    }
                }
            }
        }
    }
}

#[instrument(
    skip(state, payload, _tag),
    fields(job = to_variant_name(&payload).expect("invalid job"))
)]
async fn do_job(state: JobState, payload: Job, _tag: String) -> Result<()> {
    match payload {
        Job::Hello => {
            info!("hello monaxia!");
        }
        Job::ActivityPreprocess(json_text, validation) => {
            let next = ap::preprocess_activity(&state, json_text, validation).await?;
            state.producer.enqueue(next, None).await?;
        }
        Job::ActivityDistribution(raw_activity) => {
            ap::activity_distribution(&state, raw_activity).await?;
        }
    }
    Ok(())
}
