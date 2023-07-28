mod error;
mod extract;
mod jsonld;
mod routes;
pub mod state;

use axum::{
    http::{header::ACCEPT, Request},
    routing::{get, post},
    Router,
};
use tower_http::trace::{OnRequest, TraceLayer};
use tracing::{debug, Span};

pub fn construct_router(state_source: state::AppState) -> Router<()> {
    // routes
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

    // layers
    let trace_layer = TraceLayer::new_for_http().on_request(OnRequestHandler);

    Router::new()
        .merge(meta_router)
        .nest("/users", users_router)
        .with_state(state_source)
        .layer(trace_layer)
}

#[derive(Debug, Clone)]
struct OnRequestHandler;

impl<B> OnRequest<B> for OnRequestHandler {
    fn on_request(&mut self, request: &Request<B>, _: &Span) {
        let accept = request
            .headers()
            .get(ACCEPT)
            .map(|a| a.to_str().unwrap_or("<binary>"))
            .unwrap_or_default();
        debug!("started processing request (Accept: {accept})");
    }
}
