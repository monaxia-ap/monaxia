mod error;
mod extract;
mod jsonld;
mod routes;
pub mod state;

use crate::worker::start_workers;

use std::sync::Arc;

use anyhow::Result;
use axum::{
    http::{header::ACCEPT, Request},
    routing::{get, post},
    Router, Server,
};
use monaxia_data::config::Config;
use monaxia_job::job::{Job, MxJob};
use tokio::{select, signal};
use tower_http::trace::{OnRequest, TraceLayer};
use tracing::{debug, info, Span};

pub async fn run_server(config: Arc<Config>) -> Result<()> {
    // start workers
    let producer = start_workers(config.clone()).await?;

    // start web server
    let state = state::construct_state(config.clone(), producer.clone()).await?;
    let routes = construct_router(state);

    let bind_addr = config.server.bind;
    let server = Server::bind(&bind_addr)
        .serve(routes.into_make_service())
        .with_graceful_shutdown(shutdown());

    producer
        .enqueue(MxJob::new_single(Job::Hello), None)
        .await?;
    server.await?;
    Ok(())
}

fn construct_router(state_source: state::AppState) -> Router<()> {
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

async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("cannot hook Ctrl-C");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("cannot hook SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutting down web server");
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
