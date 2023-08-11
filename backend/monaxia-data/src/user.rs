use std::ops::RangeInclusive;

use once_cell::sync::Lazy;
use regex::Regex;
use rsa::{RsaPrivateKey, RsaPublicKey};
use thiserror::Error as ThisError;
use url::Url;

static RE_USERNAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[A-Za-z0-9_]+$"#).expect("invalid regex"));

/// Represents username format error.
/// Not covered for in-use check or prohibited patterns.
#[derive(Debug, Clone, ThisError)]
pub enum UsernameError {
    #[error("out of length range; it should be in {0:?}")]
    OutOfLength(RangeInclusive<usize>),

    #[error("non-ASCII character is prohibited")]
    NonAscii,

    #[error("only alphanumeric and underscore is allowed")]
    Inencodable,

    #[error("other error")]
    Other,
}

#[derive(Debug)]
pub struct RemoteUserRegistration {
    pub username: String,
    pub public_key: RsaPublicKey,
    pub public_key_id: String,
}

#[derive(Debug)]
pub struct LocalUserRegistration {
    pub base_url: Url,
    pub username: String,
    pub private_key: RsaPrivateKey,
}

/// Validates username format.
pub fn validate_username_format(
    input: &str,
    length_range: RangeInclusive<usize>,
) -> Result<(), UsernameError> {
    if !length_range.contains(&input.len()) {
        return Err(UsernameError::OutOfLength(length_range));
    }

    if !input.is_ascii() {
        return Err(UsernameError::NonAscii);
    }

    if !RE_USERNAME.is_match(input) {
        return Err(UsernameError::Inencodable);
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct LocalUser {
    pub id: String,
    pub id_seq: String,
    pub username: String,
    pub public_key: String,
}

#[derive(Debug, Clone)]
pub struct RemoteUser {}

#[derive(Debug, Clone)]
pub struct UserPublicKey {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalUserUrl {
    /// `/users/:user_id`
    Id,

    /// `/users/:user_id/inbox`
    Inbox,

    /// `/users/:user_id/outbox`
    Outbox,

    /// `/users/:user_id#main-key`
    KeyId,
}

pub fn generate_local_user_url(base_url: &Url, user_id: &str, ty: LocalUserUrl) -> Url {
    match ty {
        LocalUserUrl::Id => base_url.join(&format!("/users/{}", user_id)),
        LocalUserUrl::Inbox => base_url.join(&format!("/users/{}/inbox", user_id)),
        LocalUserUrl::Outbox => base_url.join(&format!("/users/{}/outbox", user_id)),
        LocalUserUrl::KeyId => base_url.join(&format!("/users/{}#main-key", user_id)),
    }
    .expect("URL error")
}
