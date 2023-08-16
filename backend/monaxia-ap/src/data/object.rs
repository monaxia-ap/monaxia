use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: Url,
    pub preferred_username: String,
    pub public_key: PersonPublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonPublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}
