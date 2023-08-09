use anyhow::Result;
use lapin::{Connection as LapinConnection, ConnectionProperties};
use monaxia_data::config::Config;
use monaxia_job::job::MxJob;
use monaxia_queue::{
    job::{Consumer, Producer},
    queue::amqp::{create_amqp_consumer, create_amqp_producer},
};

pub async fn create_queues(config: &Config) -> Result<(Producer<MxJob>, Vec<Consumer<MxJob>>)> {
    let conn = LapinConnection::connect(&config.queue.url, ConnectionProperties::default()).await?;
    let producer = create_amqp_producer(&conn, "producer").await?;
    let consumers = create_amqp_consumer(&conn, "consumer", config.queue.workers).await?;

    Ok((producer, consumers))
}
