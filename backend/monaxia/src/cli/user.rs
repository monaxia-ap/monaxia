use crate::web::state::AppState;

use anyhow::Result;
use clap::Parser;
use inquire::{validator::Validation, Text};
use monaxia_data::user::validate_username_format;
use rand::prelude::*;
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey,
};

pub const KEY_LENGTH: usize = 2048;

#[derive(Debug, Clone, Parser)]
pub enum UserSubcommand {
    /// Create new user.
    Create,
}

pub async fn execute_user_subcommand(state: AppState, subcommand: UserSubcommand) -> Result<()> {
    match subcommand {
        UserSubcommand::Create => create_user(state).await?,
    }

    Ok(())
}

async fn create_user(state: AppState) -> Result<()> {
    let username_range = 1..=(state.config.user.username_max_length);
    let banned_usernames = state.config.user.banned_usernames.clone();

    let username = Text::new("Username:")
        .with_validator(move |n: &str| {
            match validate_username_format(n, username_range.clone()) {
                Ok(()) => (),
                Err(e) => return Ok(Validation::Invalid(e.into())),
            }

            if banned_usernames.contains(&n.to_string()) {
                return Ok(Validation::Invalid("input username is banned".into()));
            }

            // TODO: check already-in-use username

            Ok(Validation::Valid)
        })
        .prompt()?;

    println!("Generating new keypair");
    let mut rng = thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, KEY_LENGTH)?;
    let public_key = private_key.to_public_key();
    let private_pkcs8_pem = private_key.to_pkcs8_pem(LineEnding::LF)?;
    let public_pkcs8_pem = public_key.to_public_key_pem(LineEnding::LF)?;

    println!("Registering {username}");
    Ok(())
}
