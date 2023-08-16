use std::{collections::HashMap, str::FromStr};

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

pub mod mime {
    pub const APPLICATION_ACTIVITY_JSON: &str = "application/activity+json";
    pub const APPLICATION_LD_JSON: &str = "application/ld+json";
}

pub mod header {
    pub const DIGEST: &str = "digest";
    pub const SIGNATURE: &str = "signature";
    pub const CANONICAL_REQUEST_TARGET: &str = "(request-target)";
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureHeader {
    pub key_id: String,
    pub algorithm: String,
    pub headers: Vec<String>,

    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
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
        let signature = STANDARD_NO_PAD
            .decode(props.remove("signature").ok_or(SignatureHeaderError)?)
            .map_err(|_| SignatureHeaderError)?;

        Ok(SignatureHeader {
            key_id,
            algorithm,
            headers,
            signature,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigestAlgorithm {
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestHeader {
    pub algorithm: DigestAlgorithm,

    #[serde(with = "serde_bytes")]
    pub digest_bytes: Vec<u8>,
}

#[derive(Debug, ThisError)]
#[error("failed to parse digest header")]
pub struct DigestHeaderError;

impl FromStr for DigestHeader {
    type Err = DigestHeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs: HashMap<&str, &str> = s
            .split(',')
            .filter_map(|p| {
                let mut kv = p.split('=');
                let key = kv.next();
                let value = kv.next();
                if let (Some(k), Some(v)) = (key, value) {
                    Some((k, v))
                } else {
                    None
                }
            })
            .collect();

        for (algorithm, digest) in pairs {
            let algorithm = match algorithm.to_ascii_lowercase().as_str() {
                "sha-256" => DigestAlgorithm::Sha256,
                "sha-512" => DigestAlgorithm::Sha512,
                _ => continue,
            };
            let digest_bytes = STANDARD_NO_PAD
                .decode(digest)
                .map_err(|_| DigestHeaderError)?;
            return Ok(DigestHeader {
                algorithm,
                digest_bytes,
            });
        }

        Err(DigestHeaderError)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
pub struct RequestValidation {
    pub digest_header: DigestHeader,
    pub signature_header: SignatureHeader,
    pub header_values: HashMap<String, String>,
}
