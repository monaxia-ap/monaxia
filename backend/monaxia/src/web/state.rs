use crate::repository_impl::construct_container_db;

use std::{path::Path, sync::Arc};

use anyhow::Result;
use monaxia_data::config::{read_config, Config};
use monaxia_db::establish_pool;
use monaxia_repository::Container;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub container: Container,
}

pub async fn construct_state(config_filename: &Path) -> Result<AppState> {
    let config = read_config(config_filename).await?;

    let pool = establish_pool(&config.database.url).await?;
    let container = construct_container_db(pool);

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
