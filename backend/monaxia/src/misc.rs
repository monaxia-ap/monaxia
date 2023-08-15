use monaxia_data::config::Config;
use reqwest::{Client, ClientBuilder, Result as ReqwestResult};

pub const SOFTWARE_NAME: &str = "monaxia";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod mime {
    pub const APPLICATION_ACTIVITY_JSON: &str = "application/activity+json";
    pub const APPLICATION_LD_JSON: &str = "application/ld+json";
}

pub mod header {
    pub const DIGEST: &str = "digest";
    pub const SIGNATURE: &str = "signature";
    pub const CANONICAL_REQUEST_TARGET: &str = "(request-target)";
}

pub fn create_http_client(config: &Config) -> ReqwestResult<Client> {
    let user_agent = format!(
        "{}/{} (+{})",
        SOFTWARE_NAME,
        VERSION,
        config.cached.server_base_url()
    );
    let http_client = ClientBuilder::new().user_agent(user_agent).build()?;
    Ok(http_client)
}
