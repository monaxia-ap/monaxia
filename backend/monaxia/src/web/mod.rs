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
        .route("/host-meta", get(meta::endpoint::host_meta))
        .route(
            "/.well-known/webfinger",
            get(meta::endpoint::wellknown_webfinger),
        )
        .route(
            "/.well-known/nodeinfo",
            get(meta::endpoint::wellknown_nodeinfo),
        )
        .route("/nodeinfo/2.1", get(meta::endpoint::nodeinfo))
        .with_state(state_source)
}
