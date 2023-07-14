mod meta {
    pub mod endpoint;
    mod schema;
}

mod error;
mod extract;
pub mod state;

use axum::{routing::get, Router};

pub fn construct_router(state_source: state::AppState) -> Router<()> {
    Router::new()
        .route("/.well-known/webfinger", get(meta::endpoint::webfinger))
        .with_state(state_source)
}
