use crate::web::error::{ErrorResponse, ErrorType};

use axum::{
    extract::{
        rejection::{FormRejection, JsonRejection, QueryRejection},
        Form, Json, Query,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;

pub type RjJson<T> = WithRejection<Json<T>, MonaxiaRejection>;
pub type RjQuery<T> = WithRejection<Query<T>, MonaxiaRejection>;
pub type RjForm<T> = WithRejection<Form<T>, MonaxiaRejection>;

/// Wrapped rejection for axum's default extractors.
pub struct MonaxiaRejection {
    status_code: StatusCode,
    reason: String,
}

impl From<JsonRejection> for MonaxiaRejection {
    fn from(rejection: JsonRejection) -> Self {
        MonaxiaRejection {
            status_code: rejection.status(),
            reason: rejection.body_text(),
        }
    }
}

impl From<QueryRejection> for MonaxiaRejection {
    fn from(rejection: QueryRejection) -> Self {
        MonaxiaRejection {
            status_code: rejection.status(),
            reason: rejection.body_text(),
        }
    }
}

impl From<FormRejection> for MonaxiaRejection {
    fn from(rejection: FormRejection) -> Self {
        MonaxiaRejection {
            status_code: rejection.status(),
            reason: rejection.body_text(),
        }
    }
}

impl IntoResponse for MonaxiaRejection {
    fn into_response(self) -> Response {
        ErrorResponse {
            status_code: self.status_code,
            error: ErrorType::InvalidRequest,
            reason: self.reason,
        }
        .into_response()
    }
}
