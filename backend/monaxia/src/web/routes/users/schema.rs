use monaxia_ap::data::jsonld::JsonLd;
use serde::Serialize;
use url::Url;

/// Response type of ActivityPub Person object.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename = "Person", rename_all = "camelCase")]
pub struct ResponsePerson {
    #[serde(flatten)]
    pub jsonld: JsonLd,

    pub id: String,
    pub preferred_username: String,
    pub discoverable: bool,
    pub inbox: Url,
    pub outbox: Url,
    pub public_key: ResponsePersonPublicKey,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePersonPublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}
