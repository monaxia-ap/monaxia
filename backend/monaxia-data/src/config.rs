use std::{net::SocketAddr, path::Path, sync::Arc};

use anyhow::{ensure, Result};
use serde::Deserialize;
use tokio::fs::read_to_string;
use url::Url;

// Top-level config file structure.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// \[server\] block.
    pub server: ConfigServer,

    /// \[database\] block.
    pub database: ConfigDatabase,

    /// \[queue\] block.
    pub queue: ConfigQueue,

    /// \[user\] block.
    pub user: ConfigUser,

    /// Contains cached properties.
    #[serde(skip)]
    pub cached: ConfigCached,
}

impl Default for Config {
    fn default() -> Self {
        let mut config = Self {
            server: Default::default(),
            database: Default::default(),
            queue: Default::default(),
            user: Default::default(),
            cached: Default::default(),
        };
        config.warmup();
        config
    }
}

impl Config {
    pub fn warmup(&mut self) {
        let cached = ConfigCached::warmup(self);
        self.cached = cached;
    }
}

/// Server configurations.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigServer {
    /// Bind address.
    pub bind: SocketAddr,

    /// Connection schema (http / https).
    pub schema: String,

    /// Incoming server Domain.
    pub domain: String,

    /// Incoming server port.
    pub port: Option<u16>,
}

impl Default for ConfigServer {
    fn default() -> Self {
        Self {
            bind: SocketAddr::V4("0.0.0.0:3000".parse().expect("invalid socket addr")),
            schema: "http".into(),
            domain: "localhost".into(),
            port: None,
        }
    }
}

/// Data configurations.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigDatabase {
    /// Database connection URL.
    pub url: String,
}

impl Default for ConfigDatabase {
    fn default() -> Self {
        Self {
            url: "postgres://localhost:5432/monaxia".into(),
        }
    }
}

/// Data configurations.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigQueue {
    /// Database connection URL.
    pub url: String,

    /// Workers count to create.
    pub workers: usize,
}

impl Default for ConfigQueue {
    fn default() -> Self {
        Self {
            url: "amqp://localhost:5672/monaxia".into(),
            workers: 4,
        }
    }
}

/// Local user registration status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRegistration {
    /// Open (free) registration.
    Open,

    /// Closed.
    Closed,

    /// Invitation only.
    Invitation,
}

/// Local user settings.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigUser {
    /// Registration status.
    pub registration: UserRegistration,

    /// Maximum length for new user.
    pub username_max_length: usize,

    /// Banned (or reserved) usernames.
    pub banned_usernames: Vec<String>,
}

impl Default for ConfigUser {
    fn default() -> Self {
        Self {
            registration: UserRegistration::Closed,
            username_max_length: 32,
            banned_usernames: vec![],
        }
    }
}

/// Cached properties based on config file.
#[derive(Debug, Clone)]
pub struct ConfigCached {
    server_base_url: Url,
}

impl Default for ConfigCached {
    fn default() -> Self {
        Self {
            server_base_url: Url::parse("http://localhost").expect("invali URL"),
        }
    }
}

impl ConfigCached {
    fn warmup(cold_config: &Config) -> ConfigCached {
        let mut server_base_url = Url::parse(&format!(
            "{}://{}",
            cold_config.server.schema, cold_config.server.domain
        ))
        .expect("invalid base URL");
        server_base_url
            .set_port(cold_config.server.port)
            .expect("invalid base URL");

        ConfigCached { server_base_url }
    }

    /// Constructs server's base URL like `https://example.com`.
    pub fn server_base_url(&self) -> &Url {
        &self.server_base_url
    }

    /// Constructs acct domain string, which may contain port part.
    /// Returned value will be like `example.com` or `localhost:3000`.
    pub fn acct_origin(&self) -> String {
        let host = self.server_base_url.host_str().expect("invalid base URL");
        if let Some(port) = self.server_base_url.port() {
            format!("{host}:{port}")
        } else {
            host.to_string()
        }
    }
}

pub async fn read_config(filename: &Path) -> Result<Arc<Config>> {
    ensure!(filename.exists(), "config file not found");
    let config_text = read_to_string(filename).await?;
    let mut config: Config = toml::from_str(&config_text)?;
    config.warmup();
    Ok(Arc::new(config))
}
