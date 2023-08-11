use super::schema::{ResponsePerson, ResponsePersonPublicKey};

use crate::web::{
    error::{map_err_queue, MxResult},
    extract::{ApJson, ApJsonText, ApValidation, MustAcceptActivityJson, PathLocalUser},
    jsonld::JSONLD_OBJECT,
    state::AppState,
};

use axum::{extract::State, http::StatusCode};
use monaxia_data::ap::RequestValidation;
use monaxia_job::job::{Job, MxJob};
use tracing::{debug, instrument};

pub async fn actor(
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

#[instrument(skip(state, ap_validation, ap_json), fields(local_user = local_user.username))]
pub async fn inbox(
    State(state): State<AppState>,
    PathLocalUser(local_user): PathLocalUser,
    ap_validation: ApValidation,
    ApJsonText(ap_json): ApJsonText,
) -> MxResult<(StatusCode, String)> {
    debug!(
        "will validate: {:?}",
        ap_validation.signature_header.headers
    );

    let validation = RequestValidation {
        digest: ap_validation.digest,
        signature_header: ap_validation.signature_header,
        header_values: ap_validation.header_values,
    };

    state
        .producer
        .enqueue(
            MxJob::new_single(Job::ActivityPreprocess(ap_json, validation)),
            None,
        )
        .await
        .map_err(map_err_queue)?;

    Ok((StatusCode::OK, "".into()))
}

#[instrument(skip(_state), fields(local_user = local_user.username))]
pub async fn outbox(
    State(_state): State<AppState>,
    _: MustAcceptActivityJson,
    PathLocalUser(local_user): PathLocalUser,
) -> MxResult<(StatusCode, String)> {
    Ok((StatusCode::NOT_IMPLEMENTED, "not implemented yet".into()))
}
