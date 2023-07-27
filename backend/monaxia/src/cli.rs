mod migrate;
mod user;

use self::{
    migrate::{execute_migrate_subcommand, MigrateSubcommand},
    user::{execute_user_subcommand, UserSubcommand},
};
use crate::web::{construct_router, state::construct_state};

use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;
use axum::Server;
use clap::Parser;

/// ActivityPub compatible microblogging platform.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
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
    #[clap(short, long, default_value = "./config.toml")]
    pub config: PathBuf,
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

    /// Database migration.
    Migrate(MigrateSubcommand),
}

pub async fn execute_cli(args: Arguments) -> Result<()> {
    let state = construct_state(&args.options.config).await?;

    match args.subcommand {
        Subcommand::Serve { bind } => {
            let routes = construct_router(state);
            Server::bind(&bind)
                .serve(routes.into_make_service())
                .await?;
        }
        Subcommand::User(s) => execute_user_subcommand(state, s).await?,
        Subcommand::Migrate(s) => execute_migrate_subcommand(state, s).await?,
    }
    Ok(())
}
