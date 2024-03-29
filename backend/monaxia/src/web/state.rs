use crate::repository_impl::construct_container_db;

use std::sync::Arc;

use anyhow::Result;
use monaxia_data::config::Config;
use monaxia_job::job::MxJob;
use monaxia_queue::job::Producer;
use monaxia_repository::Container;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub producer: Producer<MxJob>,
    pub container: Container,
}

pub async fn construct_state(config: Config, producer: Producer<MxJob>) -> Result<AppState> {
    let container = construct_container_db(&config).await?;

    Ok(AppState {
        config: Arc::new(config),
        producer,
        container,
    })
}

#[cfg(test)]
pub fn construct_state_test() -> AppState {
    use crate::{repository_impl::construct_container_test, worker::create_test_queues};

    let (producer, _) = create_test_queues();
    let config = Default::default();
    let container = construct_container_test();

    AppState {
        config: Arc::new(config),
        producer,
        container,
    }
}
