#![allow(dead_code)]

use crate::web::error::{ErrorResponse, ErrorType};

use axum::{
    extract::{
        rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection},
        Form, Json, Path, Query,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;

pub type RjJson<T> = WithRejection<Json<T>, MonaxiaRejection>;
pub type RjQuery<T> = WithRejection<Query<T>, MonaxiaRejection>;
pub type RjForm<T> = WithRejection<Form<T>, MonaxiaRejection>;
pub type RjPath<T> = WithRejection<Path<T>, MonaxiaRejection>;

/// Wrapped rejection for axum's default extractors.
pub struct MonaxiaRejection {
    status_code: StatusCode,
    reason: String,
}

impl MonaxiaRejection {
    pub fn into_mx_error(self, error: ErrorType) -> ErrorResponse {
        ErrorResponse {
            status_code: self.status_code,
            error,
            reason: self.reason,
        }
    }
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

impl From<PathRejection> for MonaxiaRejection {
    fn from(rejection: PathRejection) -> Self {
        MonaxiaRejection {
            status_code: rejection.status(),
            reason: rejection.body_text(),
        }
    }
}

impl IntoResponse for MonaxiaRejection {
    fn into_response(self) -> Response {
        self.into_mx_error(ErrorType::InvalidRequest)
            .into_response()
    }
}
