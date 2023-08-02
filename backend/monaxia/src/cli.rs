mod migrate;
mod user;

use self::{
    migrate::{execute_migrate_subcommand, MigrateSubcommand},
    user::{execute_user_subcommand, UserSubcommand},
};
use crate::web::run_server;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use monaxia_data::config::read_config;

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
    Serve,

    /// User manipulation.
    #[clap(subcommand)]
    User(UserSubcommand),

    /// Database migration.
    Migrate(MigrateSubcommand),
}

pub async fn execute_cli(args: Arguments) -> Result<()> {
    let config = read_config(&args.options.config).await?;

    match args.subcommand {
        Subcommand::Serve => run_server(config).await?,
        Subcommand::User(s) => execute_user_subcommand(config, s).await?,
        Subcommand::Migrate(s) => execute_migrate_subcommand(config, s).await?,
    }
    Ok(())
}
