use crate::repository_impl::construct_container_db;

use std::sync::Arc;

use anyhow::Result;
use monaxia_data::config::Config;
use monaxia_repository::Container;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub container: Container,
}

pub async fn construct_state(config: Config) -> Result<AppState> {
    let container = construct_container_db(&config).await?;

    Ok(AppState {
        config: Arc::new(config),
        container,
    })
}

#[cfg(test)]
pub fn construct_state_test() -> AppState {
    use crate::repository_impl::construct_container_test;

    let config = Default::default();
    let container = construct_container_test();

    AppState {
        config: Arc::new(config),
        container,
    }
}
