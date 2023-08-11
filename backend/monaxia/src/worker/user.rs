use anyhow::Result;
use monaxia_data::user::UserPublicKey;

pub async fn retrieve_public_key(key_id: &str) -> Result<UserPublicKey> {
    // attention: AUTHORIZED_FETCH
    // * Mastodon never provides even Actor's public key for unsigned requests.

    todo!();
}
