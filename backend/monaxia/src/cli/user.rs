use crate::web::state::AppState;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tokio::fs::read_to_string;

#[derive(Debug, Clone, Parser)]
pub enum UserSubcommand {
    /// Create new user.
    Create {
        /// Username. Used as preferredUsername and display name.
        username: String,

        /// RSA private key file.
        private_key: PathBuf,

        /// RSA public key file.
        public_key: PathBuf,
    },
}

pub async fn execute_user_subcommand(state: AppState, subcommand: UserSubcommand) -> Result<()> {
    match subcommand {
        UserSubcommand::Create {
            username,
            private_key,
            public_key,
        } => {
            println!("Creating user {username}");
            let private_pem = read_to_string(private_key).await?;
            let public_pem = read_to_string(public_key).await?;
            println!("Private Key:\n{private_pem}");
            println!("Public Key:\n{public_pem}");
        }
    }
    Ok(())
}
