use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};

use axum_extra::extract::WithRejection;
use monaxia_data::user::LocalUser;
use serde::Deserialize;

use crate::{
    repository::r#trait::user::UserFind,
    web::{
        error::{map_err_repository, ErrorResponse, ErrorType},
        state::AppState,
    },
};

use super::reject::RjPath;

#[derive(Debug, Clone)]
pub struct PathLocalUser(pub LocalUser);

#[async_trait]
impl FromRequestParts<AppState> for PathLocalUser {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let WithRejection(Path(PathUserId { user_id }), _) =
            RjPath::<PathUserId>::from_request_parts(parts, state)
                .await
                .map_err(|e| e.into_mx_error(ErrorType::InvalidRequest))?;
        let local_user = state
            .container
            .user
            .find_local_user(UserFind::UserId(&user_id))
            .await
            .map_err(map_err_repository)?;
        let Some(local_user) = local_user else {
            return Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: ErrorType::NotFound,
                reason: format!("local user {user_id} not found")
            });
        };

        Ok(PathLocalUser(local_user))
    }
}

#[derive(Debug, Deserialize)]
struct PathUserId {
    pub user_id: String,
}
