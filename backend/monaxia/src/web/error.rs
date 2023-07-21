use crate::repository::RepoError;

use anyhow::Error as AnyhowError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub type MxResult<T> = Result<T, ErrorResponse>;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorType {
    /// Request extractor failed.
    InvalidRequest,
    OtherError,
}

/// JSON structure of generic error response.
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub error: ErrorType,
    pub reason: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

pub fn bail_other<T>(status_code: StatusCode, reason: impl Into<String>) -> MxResult<T> {
    Err(ErrorResponse {
        status_code,
        error: ErrorType::OtherError,
        reason: reason.into(),
    })
}

pub fn map_err_anyhow(err: AnyhowError) -> ErrorResponse {
    ErrorResponse {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error: ErrorType::OtherError,
        reason: err.to_string(),
    }
}

pub fn map_err_repository(err: RepoError) -> ErrorResponse {
    ErrorResponse {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error: ErrorType::OtherError,
        reason: err.to_string(),
    }
}
