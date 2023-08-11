use super::JobState;
use crate::constant::mime::APPLICATION_ACTIVITY_JSON;

use anyhow::{bail, Result};
use monaxia_data::{
    ap::object::Person,
    user::{RemoteUserRegistration, UserPublicKey},
};
use monaxia_repository::repo::user::UserFind;
use reqwest::header::ACCEPT;
use rsa::{pkcs8::DecodePublicKey, RsaPublicKey};
use tracing::error;

pub(super) async fn retrieve_public_key(state: &JobState, key_id: &str) -> Result<UserPublicKey> {
    // attention: AUTHORIZED_FETCH
    // * Mastodon never provides even Actor's public key for unsigned requests.

    let user = state
        .container
        .user
        .find_local_user(UserFind::KeyId(key_id))
        .await?;
    if let Some(user) = user {
        return Ok(user.public_key);
    }

    // unknown remote user
    let resp = state
        .http_client
        .get(key_id)
        .header(ACCEPT, APPLICATION_ACTIVITY_JSON)
        .send()
        .await?;
    if !resp.status().is_success() {
        error!(
            "failed to fetch public key: {key_id} (status: {})",
            resp.status()
        );
        bail!("public key fetch error");
    }

    let Ok(remote_person): Result<Person, _> = resp.json().await else {
        error!("invalid Person object detected");
        bail!("public key fetch error");
    };
    let Some(remote_domain) = remote_person.id.domain() else {
        error!("malformed id detected: {}", remote_person.id);
        bail!("public key fetch error");
    };
    let Ok(public_key) = RsaPublicKey::from_public_key_pem(&remote_person.public_key.public_key_pem) else {
        error!("failed to parse public key PEM: {}", remote_person.id);
        bail!("public key fetch error");
    };

    let user = state
        .container
        .user
        .register_remote_user(
            RemoteUserRegistration {
                username: remote_person.preferred_username,
                public_key,
                public_key_id: remote_person.public_key.id,
            },
            remote_domain,
        )
        .await?;

    Ok(user.public_key)
}
