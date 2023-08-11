use std::collections::HashMap;

use crate::{
    constant::{
        header::{CANONICAL_REQUEST_TARGET, DIGEST, SIGNATURE},
        mime::{APPLICATION_ACTIVITY_JSON, APPLICATION_LD_JSON},
    },
    web::error::{bail_err_header, map_err_extract, ErrorResponse, ErrorType},
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
use monaxia_data::http::SignatureHeader;
use serde::Serialize;

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

/// Accepts only activity JSON, but don't deserialize.
#[derive(Debug, Clone)]
#[must_use]
pub struct ApJsonText(pub String);

#[async_trait]
impl<S, B> FromRequest<S, B> for ApJsonText
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        if ap_json_content_type(req.headers()) {
            let inner = String::from_request(req, state).await;
            match inner {
                Ok(s) => Ok(ApJsonText(s)),
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

/// Accepts only activity JSON and deserializes it.
#[derive(Debug, Clone)]
#[must_use]
pub struct ApJson<T>(pub T);

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

/// Parses signature and digest header.
#[derive(Debug, Clone)]
#[must_use]
pub struct ApValidation {
    pub digest: String,
    pub signature_header: SignatureHeader,
    pub header_values: HashMap<String, String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for ApValidation
where
    S: Send + Sync,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // digest header
        let digest = {
            let Some(digest) = parts.headers.get(DIGEST) else {
                return bail_err_header(DIGEST)?;
            };
            digest.to_str().map_err(map_err_extract)?.to_string()
        };

        // signature header extraction
        let signature_header: SignatureHeader = {
            let Some(signature_header_str) = parts.headers.get(SIGNATURE) else {
                return  bail_err_header(SIGNATURE)?;
            };
            let signature_header_str = signature_header_str
                .to_str()
                .map_err(map_err_extract)?
                .to_string();
            signature_header_str.parse().map_err(map_err_extract)?
        };

        // header values
        let mut header_values = HashMap::new();
        for header_name in &signature_header.headers {
            match header_name.as_str() {
                CANONICAL_REQUEST_TARGET => {
                    let value = format!(
                        "{} {}",
                        parts.method.to_string().to_lowercase(),
                        parts
                            .uri
                            .path_and_query()
                            .map(|pq| pq.as_str())
                            .unwrap_or("/")
                    );
                    header_values.insert(CANONICAL_REQUEST_TARGET.into(), value);
                }
                header => {
                    let Some(value) = parts.headers.get(header) else {
                        return bail_err_header(header)?;
                    };
                    let value = value.to_str().map_err(map_err_extract)?.to_string();
                    header_values.insert(header.to_string(), value);
                }
            }
        }

        Ok(ApValidation {
            digest,
            signature_header,
            header_values,
        })
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
