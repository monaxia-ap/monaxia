use super::schema::ShowResponse;

use crate::web::{
    error::MxResult,
    extract::{ApJson, MustAcceptActivityJson},
    jsonld::JSONLD_OBJECT,
    state::AppState,
};

use axum::extract::State;

pub async fn show(
    State(state): State<AppState>,
    _: MustAcceptActivityJson,
) -> MxResult<ApJson<ShowResponse>> {
    Ok(ApJson(ShowResponse {
        jsonld: JSONLD_OBJECT.clone(),
    }))
}
