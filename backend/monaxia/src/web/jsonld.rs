use std::fmt::{Formatter, Result as FmtResult};

use once_cell::sync::Lazy;
use serde::{
    de::{value::MapAccessDeserializer, Error as SerdeDeError, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value as JsonValue;
use url::Url;

pub static JSONLD_OBJECT: Lazy<JsonLd> = Lazy::new(|| JsonLd {
    context: vec![
        JsonLdContext::Url(
            Url::parse("https://www.w3.org/ns/activitystreams").expect("invalid context"),
        ),
        JsonLdContext::Url(Url::parse("https://w3id.org/security/v1").expect("invalid context")),
    ],
});

/// Contains `@context` property. supposed to used with `#[serde(flatten)]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLd {
    #[serde(rename = "@context")]
    pub context: Vec<JsonLdContext>,
}

#[derive(Debug, Clone)]
pub enum JsonLdContext {
    Url(Url),
    Object(JsonValue),
}

impl Serialize for JsonLdContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JsonLdContext::Url(url) => serializer.serialize_str(url.as_str()),
            JsonLdContext::Object(object) => object.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for JsonLdContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextVisitor;
        impl<'de> Visitor<'de> for ContextVisitor {
            type Value = JsonLdContext;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<JsonLdContext, E>
            where
                E: SerdeDeError,
            {
                let url = Url::parse(value).map_err(E::custom)?;
                Ok(JsonLdContext::Url(url))
            }

            fn visit_map<M>(self, map: M) -> Result<JsonLdContext, M::Error>
            where
                M: MapAccess<'de>,
            {
                let object: JsonValue = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
                Ok(JsonLdContext::Object(object))
            }
        }

        deserializer.deserialize_any(ContextVisitor)
    }
}
