use super::schema::{ResponsePerson, ResponsePersonPublicKey};

use crate::web::{
    error::MxResult,
    extract::{ApJson, MustAcceptActivityJson, PathLocalUser},
    jsonld::JSONLD_OBJECT,
    state::AppState,
};

use axum::extract::State;

pub async fn show(
    State(state): State<AppState>,
    _: MustAcceptActivityJson,
    PathLocalUser(local_user): PathLocalUser,
) -> MxResult<ApJson<ResponsePerson>> {
    let base_url = state.config.cached.server_base_url();
    let id_url = base_url
        .join(&format!("/users/{}", local_user.id))
        .expect("URL error");
    let inbox_url = base_url
        .join(&format!("/users/{}/inbox", local_user.id))
        .expect("URL error");
    let outbox_url = base_url
        .join(&format!("/users/{}/outbox", local_user.id))
        .expect("URL error");
    let pubkey_id = {
        let mut url = id_url.clone();
        url.set_fragment(Some("main-key"));
        url
    };

    Ok(ApJson(ResponsePerson {
        jsonld: JSONLD_OBJECT.clone(),
        id: id_url.to_string(),
        preferred_username: local_user.username.clone(),
        discoverable: true,
        inbox: inbox_url,
        outbox: outbox_url,
        public_key: ResponsePersonPublicKey {
            id: pubkey_id.to_string(),
            owner: id_url.to_string(),
            public_key_pem: local_user.public_key,
        },
    }))
}
