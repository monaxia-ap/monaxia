use crate::{
    constant::mime::{APPLICATION_ACTIVITY_JSON, APPLICATION_LD_JSON},
    web::error::{ErrorResponse, ErrorType},
};

use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{FromRequest, FromRequestParts},
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        request::Parts,
        HeaderMap, HeaderValue, Request, StatusCode,
    },
    response::{IntoResponse, Response},
    BoxError, Json,
};
use mime::{Mime, APPLICATION_JSON, TEXT_HTML};
use serde::{de::DeserializeOwned, Serialize};

/// Accept header type.
#[derive(Debug, Clone, Copy)]
pub enum ApAccept {
    /// `application/activity+json`.
    ActivityJson,

    /// `text/html`.
    Html,
}

/// Checks `Accept` header and only accepts ActivityPub or Web requests.
#[derive(Debug, Clone)]
#[must_use]
pub struct ApDualAccept(pub ApAccept);

#[async_trait]
impl<S> FromRequestParts<S> for ApDualAccept
where
    S: Send + Sync,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let accept = ap_accept(&parts.headers);
        Ok(ApDualAccept(accept))
    }
}

/// Checks `Accept` header and only accepts ActivityPub requests.
#[derive(Debug, Clone, Copy)]
#[must_use]
pub struct MustAcceptActivityJson;

#[async_trait]
impl<S> FromRequestParts<S> for MustAcceptActivityJson
where
    S: Send + Sync,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match ap_accept(&parts.headers) {
            ApAccept::ActivityJson => Ok(MustAcceptActivityJson),
            _ => Err(ErrorResponse {
                status_code: StatusCode::NOT_ACCEPTABLE,
                error: ErrorType::InvalidRequest,
                reason: "must accept application/activity+json".into(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
#[must_use]
pub struct ApJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ApJson<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        if ap_json_content_type(req.headers()) {
            let inner = Json::<T>::from_request(req, state).await;
            match inner {
                Ok(Json(d)) => Ok(ApJson(d)),
                Err(r) => Err(ErrorResponse {
                    status_code: r.status(),
                    error: ErrorType::InvalidRequest,
                    reason: r.body_text(),
                }),
            }
        } else {
            Err(ErrorResponse {
                error: ErrorType::MissingContentType,
                reason: "Content-Type must be application/activity+json".into(),
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
            })
        }
    }
}

impl<T> IntoResponse for ApJson<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut response = Json(self.0).into_response();
        let headers = response.headers_mut();
        headers.remove(CONTENT_TYPE);
        headers.append(
            CONTENT_TYPE,
            HeaderValue::from_str(APPLICATION_ACTIVITY_JSON).expect("invalid header value"),
        );

        response
    }
}

fn ap_accept(headers: &HeaderMap) -> ApAccept {
    let Some(accept) = headers.get(ACCEPT) else {
        return ApAccept::Html;
    };
    let Ok(accept_str) = accept.to_str() else {
        return ApAccept::Html;
    };

    let mimes = accept_str.split(',').map(|a| a.trim());
    for mime_str in mimes {
        let Ok(mime) = mime_str.parse::<Mime>() else {
            continue;
        };
        if mime == APPLICATION_ACTIVITY_JSON
            || mime == APPLICATION_LD_JSON
            || mime == APPLICATION_JSON
        {
            return ApAccept::ActivityJson;
        } else if mime == TEXT_HTML {
            return ApAccept::Html;
        }
    }

    ApAccept::Html
}

fn ap_json_content_type(headers: &HeaderMap) -> bool {
    let Some(content_type) = headers.get(CONTENT_TYPE) else {
        return false;
    };
    let Ok(content_type) = content_type.to_str() else {
        return false;
    };
    let Ok(mime) = content_type.parse::<Mime>() else {
        return false;
    };

    mime == APPLICATION_ACTIVITY_JSON || mime == APPLICATION_JSON
}
