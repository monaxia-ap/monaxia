mod cli;
mod web;

use crate::cli::{Subcommand, UserSubcommand};

use anyhow::Result;
use axum::Server;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Arguments::parse();
    tracing_subscriber::fmt::init();

    match args.subcommand {
        Subcommand::Serve { bind } => {
            let state = web::state::AppState {};
            let routes = web::construct_router(state);
            Server::bind(&bind)
                .serve(routes.into_make_service())
                .await?;
        }
        Subcommand::User(s) => match s {
            UserSubcommand::Create {
                username,
                private_key,
                public_key,
            } => {
                println!("Creating user {username}");
            }
        },
    }

    Ok(())
}
