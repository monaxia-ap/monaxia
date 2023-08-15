use super::jsonld::JsonLd;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawActivity {
    #[serde(flatten)]
    pub jsonld: JsonLd,

    /// Activity type.
    #[serde(rename = "type")]
    pub ty: String,

    /// Activity ID. Not present if transient.
    pub id: Option<String>,

    /// Activity actor.
    pub actor: Option<JsonValue>,

    /// Activity object.
    pub object: Option<JsonValue>,
}
