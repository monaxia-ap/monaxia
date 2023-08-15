use crate::{misc::create_http_client, repository_impl::construct_container_db};

use std::sync::Arc;

use anyhow::Result;
use monaxia_data::config::Config;
use monaxia_job::job::MxJob;
use monaxia_queue::job::Producer;
use monaxia_repository::Container;
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub producer: Producer<MxJob>,
    pub container: Container,
    pub http_client: Client,
}

pub async fn construct_state(config: Arc<Config>, producer: Producer<MxJob>) -> Result<AppState> {
    let container = construct_container_db(&config).await?;
    let http_client = create_http_client()?;

    Ok(AppState {
        config,
        producer,
        container,
        http_client,
    })
}

#[cfg(test)]
pub fn construct_state_test() -> AppState {
    use crate::{repository_impl::construct_container_test, worker::create_test_queues};

    let (producer, _) = create_test_queues();
    let config = Default::default();
    let container = construct_container_test();
    let http_client = create_http_client().expect("invalid client");

    AppState {
        config: Arc::new(config),
        producer,
        container,
        http_client,
    }
}
