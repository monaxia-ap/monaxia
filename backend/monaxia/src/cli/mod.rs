mod mx;
mod user;

use self::{
    mx::{execute_mx_subcommand, MxSubcommand},
    user::{execute_user_subcommand, UserSubcommand},
};
use crate::web;

use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;
use axum::Server;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    /// Specify config file path.
    #[clap(flatten)]
    pub options: CommonOptions,

    /// Target subcommand.
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

/// Common options.
#[derive(Debug, Clone, Parser)]
pub struct CommonOptions {
    /// Specify config file path.
    #[clap(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Clone, Parser)]
pub enum Subcommand {
    /// Start server.
    Serve {
        #[clap(short, long, default_value = "0.0.0.0:3000")]
        bind: SocketAddr,
    },

    /// User manipulation.
    #[clap(subcommand)]
    User(UserSubcommand),

    /// developer options.
    #[clap(subcommand)]
    Mx(MxSubcommand),
}

pub async fn execute_cli(args: Arguments) -> Result<()> {
    match args.subcommand {
        Subcommand::Serve { bind } => {
            let state = web::state::AppState {};
            let routes = web::construct_router(state);
            Server::bind(&bind)
                .serve(routes.into_make_service())
                .await?;
        }
        Subcommand::User(s) => execute_user_subcommand(args.options, s).await?,
        Subcommand::Mx(s) => execute_mx_subcommand(args.options, s).await?,
    }
    Ok(())
}
