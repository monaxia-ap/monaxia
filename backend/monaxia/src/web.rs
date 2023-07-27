mod error;
mod extract;
mod jsonld;
mod routes;
pub mod state;

use axum::{
    routing::{get, post},
    Router,
};

pub fn construct_router(state_source: state::AppState) -> Router<()> {
    let meta_router = Router::new()
        .route("/host-meta", get(routes::meta::host_meta))
        .route(
            "/.well-known/webfinger",
            get(routes::meta::wellknown_webfinger),
        )
        .route(
            "/.well-known/nodeinfo",
            get(routes::meta::wellknown_nodeinfo),
        )
        .route("/nodeinfo/2.1", get(routes::meta::nodeinfo));

    let users_router = Router::new()
        .route("/:user_id", get(routes::users::actor))
        .route("/:user_id/inbox", post(routes::users::inbox))
        .route("/:user_id/outbox", get(routes::users::outbox));

    Router::new()
        .merge(meta_router)
        .nest("/users", users_router)
        .with_state(state_source)
}
