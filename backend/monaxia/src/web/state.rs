use crate::{
    config::{read_config, Config},
    db::establish_pool,
    repository::{construct_container_db, Container},
};

use std::{path::Path, sync::Arc};

use anyhow::Result;

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
