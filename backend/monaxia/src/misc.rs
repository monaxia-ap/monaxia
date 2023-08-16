use monaxia_data::config::Config;
use reqwest::{Client, ClientBuilder, Result as ReqwestResult};

pub const SOFTWARE_NAME: &str = "monaxia";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


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
