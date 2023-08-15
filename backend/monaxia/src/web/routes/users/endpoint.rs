use super::schema::{ResponsePerson, ResponsePersonPublicKey};
use crate::web::{
    error::{map_err_queue, MxResult},
    extract::{ApJson, ApJsonText, ApValidation, MustAcceptActivityJson, PathLocalUser},
    state::AppState,
};

use axum::{extract::State, http::StatusCode};
use monaxia_data::{
    ap::{jsonld::JSONLD_OBJECT, RequestValidation},
    user::{generate_local_user_url, LocalUserUrl},
};
use monaxia_job::job::{Job, MxJob};
use serde_variant::to_variant_name;
use tracing::{debug, instrument};

pub async fn actor(
    State(state): State<AppState>,
    _: MustAcceptActivityJson,
    PathLocalUser(local_user): PathLocalUser,
) -> MxResult<ApJson<ResponsePerson>> {
    let base_url = state.config.cached.server_base_url();
    let id_url = generate_local_user_url(base_url, &local_user.id, LocalUserUrl::Id);
    let inbox_url = generate_local_user_url(base_url, &local_user.id, LocalUserUrl::Inbox);
    let outbox_url = generate_local_user_url(base_url, &local_user.id, LocalUserUrl::Outbox);

    Ok(ApJson(ResponsePerson {
        jsonld: JSONLD_OBJECT.clone(),
        id: id_url.to_string(),
        preferred_username: local_user.username.clone(),
        discoverable: true,
        inbox: inbox_url,
        outbox: outbox_url,
        public_key: ResponsePersonPublicKey {
            owner: id_url.to_string(),
            id: local_user.public_key.key_id,
            public_key_pem: local_user.public_key.key_pem,
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
    let validation = RequestValidation {
        digest_header: ap_validation.digest_header,
        signature_header: ap_validation.signature_header,
        header_values: ap_validation.header_values,
    };
    let job = Job::ActivityPreprocess(ap_json, validation);

    debug!("enqueuing {}", to_variant_name(&job).expect("invalid job"));
    state
        .producer
        .enqueue(MxJob::new_single(job), None)
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
