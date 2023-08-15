use reqwest::{Client, ClientBuilder, Result as ReqwestResult};

pub const SOFTWARE_NAME: &str = "monaxia";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const USER_AGENT: &str = concat!("monaxia/", env!("CARGO_PKG_VERSION"));
// pub const VERSION_TAG: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_COMMIT_HASH"));

pub mod mime {
    pub const APPLICATION_ACTIVITY_JSON: &str = "application/activity+json";
    pub const APPLICATION_LD_JSON: &str = "application/ld+json";
}

pub mod header {
    pub const DIGEST: &str = "digest";
    pub const SIGNATURE: &str = "signature";
    pub const CANONICAL_REQUEST_TARGET: &str = "(request-target)";
}

pub fn create_http_client() -> ReqwestResult<Client> {
    let http_client = ClientBuilder::new().user_agent(USER_AGENT).build()?;
    Ok(http_client)
}
