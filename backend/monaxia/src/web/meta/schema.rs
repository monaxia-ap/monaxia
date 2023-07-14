use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct WebfigerQuery {
    pub resource: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebfingerResponse {
    pub subject: String,
    pub links: Vec<WebfingerLink>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebfingerLink {
    pub rel: String,
    pub r#type: String,
    pub href: Url,
}
