pub mod activity;
pub mod jsonld;
pub mod object;

use serde::de::{Deserialize, DeserializeOwned, Deserializer, Error as SerdeDeError};
use serde_json::Value as JsonValue;

fn single_sequence<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let base_value = JsonValue::deserialize(deserializer)?;
    match base_value {
        JsonValue::Array(array) => {
            let sequence = array
                .into_iter()
                .map(serde_json::from_value)
                .collect::<Result<Vec<_>, _>>()
                .map_err(SerdeDeError::custom)?;
            Ok(sequence)
        }
        otherwise => {
            let single_value = serde_json::from_value(otherwise).map_err(SerdeDeError::custom)?;
            Ok(vec![single_value])
        }
    }
}
