use super::schema::{WebfigerQuery, WebfingerResponse};
use crate::web::{
    error::{bail_other, MxResult},
    extract::RjQuery,
};

use axum::{extract::Query, http::StatusCode, Json};
use axum_extra::extract::WithRejection;

pub async fn webfinger(
    WithRejection(Query(query), _): RjQuery<WebfigerQuery>,
) -> MxResult<Json<WebfingerResponse>> {
    bail_other(
        StatusCode::NOT_FOUND,
        format!("user {} not found", query.resource),
    )
}
