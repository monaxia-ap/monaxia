use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    /// Specify config file path.
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// Target subcommand.
    #[clap(subcommand)]
    pub subcommand: Subcommand,
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
}

#[derive(Debug, Clone, Parser)]
pub enum UserSubcommand {
    /// Create new user.
    Create {
        username: String,
        private_key: PathBuf,
        public_key: PathBuf,
    },
}
