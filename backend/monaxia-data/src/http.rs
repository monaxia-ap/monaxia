use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureHeader {
    pub key_id: String,
    pub algorithm: String,
    pub headers: Vec<String>,
    pub signature: String,
}

#[derive(Debug, ThisError)]
#[error("failed to parse signature header")]
pub struct SignatureHeaderError;

impl FromStr for SignatureHeader {
    type Err = SignatureHeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut props: HashMap<&str, &str> = s
            .split(',')
            .filter_map(|p| {
                let mut kv = p.split('=');
                let key = kv.next();
                let value = kv.next();
                if let (Some(k), Some(v)) = (key, value) {
                    Some((k, v.trim_matches('"')))
                } else {
                    None
                }
            })
            .collect();

        let key_id = props
            .remove("keyId")
            .ok_or(SignatureHeaderError)?
            .to_string();
        let algorithm = props
            .remove("algorithm")
            .ok_or(SignatureHeaderError)?
            .to_string();
        let headers = props
            .remove("headers")
            .ok_or(SignatureHeaderError)?
            .to_string()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let signature = props
            .remove("signature")
            .ok_or(SignatureHeaderError)?
            .to_string();

        Ok(SignatureHeader {
            key_id,
            algorithm,
            headers,
            signature,
        })
    }
}
