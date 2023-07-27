use serde::Serialize;

use crate::web::jsonld::JsonLd;

#[derive(Debug, Clone, Serialize)]
pub struct ShowResponse {
    #[serde(flatten)]
    pub jsonld: JsonLd,
}
