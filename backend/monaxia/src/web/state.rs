use crate::config::{read_config, Config};

use std::{path::Path, sync::Arc};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}

pub async fn construct_state(config_filename: &Path) -> Result<AppState> {
    let config = read_config(config_filename).await?;

    Ok(AppState {
        config: Arc::new(config),
    })
}
