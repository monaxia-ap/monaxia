use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub user: ConfigUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigServer {
    pub domain: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigUser {
    pub registration: UserRegistration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRegistration {
    Open,
    Closed,
    Invitation,
}

pub async fn read_config(filename: &Path) -> Result<Config> {
    let config_text = tokio::fs::read_to_string(filename).await?;
    let config = toml::from_str(&config_text)?;
    Ok(config)
}
