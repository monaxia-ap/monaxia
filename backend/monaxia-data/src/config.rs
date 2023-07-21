use std::{net::SocketAddr, path::Path};

use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use tokio::fs::read_to_string;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub database: ConfigDatabase,
    pub user: ConfigUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigServer {
    pub bind: SocketAddr,
    pub schema: String,
    pub domain: String,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigDatabase {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigUser {
    pub registration: UserRegistration,
    pub username_max_length: usize,
    pub banned_usernames: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRegistration {
    Open,
    Closed,
    Invitation,
}

impl ConfigServer {
    pub fn server_base_url(&self) -> Result<Url> {
        let mut url = Url::parse(&format!("{}://{}", self.schema, self.domain))?;
        url.set_port(self.port).ok().context("invalid base URL")?;
        Ok(url)
    }
}

pub async fn read_config(filename: &Path) -> Result<Config> {
    ensure!(filename.exists(), "config file not found");
    let config_text = read_to_string(filename).await?;
    let config = toml::from_str(&config_text)?;
    Ok(config)
}

pub fn make_default_config() -> Config {
    Config {
        database: ConfigDatabase {
            url: "postgres://localhost:5432/monaxia".into(),
        },
        server: ConfigServer {
            bind: SocketAddr::V4("0.0.0.0:3000".parse().expect("invalid socket addr")),
            schema: "http".into(),
            domain: "localhost".into(),
            port: None,
        },
        user: ConfigUser {
            registration: UserRegistration::Closed,
            username_max_length: 32,
            banned_usernames: vec![],
        },
    }
}
