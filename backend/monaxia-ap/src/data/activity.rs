use super::jsonld::JsonLd;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub mod ty {
    pub const ACCEPT: &str = "Accept";
    pub const UNDO: &str = "Undo";
    pub const FOLLOW: &str = "Follow";
    pub const CREATE: &str = "Create";
    pub const DELETE: &str = "Delete";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawActivity {
    #[serde(flatten)]
    pub jsonld: JsonLd,

    #[serde(rename = "type")]
    pub ty: String,

    #[serde(flatten)]
    pub rest: JsonValue,
}
