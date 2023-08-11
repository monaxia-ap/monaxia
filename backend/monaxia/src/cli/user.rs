use crate::repository_impl::construct_container_db;

use std::sync::Arc;

use anyhow::{bail, Result};
use clap::Parser;
use inquire::{validator::Validation, Confirm, Text};
use monaxia_data::{
    config::Config,
    user::{validate_username_format, LocalUserRegistration},
};
use monaxia_repository::Container;
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

pub async fn execute_user_subcommand(
    config: Arc<Config>,
    subcommand: UserSubcommand,
) -> Result<()> {
    let container = construct_container_db(&config).await?;
    match subcommand {
        UserSubcommand::Create => create_user(config, container).await?,
    }

    Ok(())
}

async fn create_user(config: Arc<Config>, container: Container) -> Result<()> {
    let username_range = 1..=(config.user.username_max_length);
    let username = Text::new("Username:")
        .with_validator(move |n: &str| {
            Ok(validate_username_format(n, username_range.clone())
                .map_or_else(|e| Validation::Invalid(e.into()), |_| Validation::Valid))
        })
        .prompt()?;

    println!("Checking whether the username is available...");
    let banned_usernames = config.user.banned_usernames.clone();
    if banned_usernames.contains(&username) {
        bail!("Username {username} is banned by setting");
    }

    if container.user.local_user_occupied(&username).await? {
        bail!("Username {username} is already taken");
    }

    println!("Generating new keypair...");
    let mut rng = thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, KEY_LENGTH)?;
    let public_key = private_key.to_public_key();
    let private_pkcs8_pem = private_key.to_pkcs8_pem(LineEnding::LF)?;
    let public_pkcs8_pem = public_key.to_public_key_pem(LineEnding::LF)?;

    println!("Generated keypair is below:");
    println!("Private Key ~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!();
    println!("{}", private_pkcs8_pem.as_str());
    println!("Public Key ~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!();
    println!("{public_pkcs8_pem}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    match Confirm::new("You may save keypairs in case of salvaging user data. Proceed?").prompt() {
        Ok(true) => (),
        _ => bail!("User creation aborted"),
    }

    let local_origin = config.cached.acct_origin();
    container.domain.acknowledge(&local_origin).await?;
    let user_id = container
        .user
        .register_local_user(
            LocalUserRegistration {
                base_url: config.cached.server_base_url().clone(),
                username,
                private_key,
            },
            &local_origin,
        )
        .await?;

    println!("Registered successfully!");
    println!("User ID is {user_id}");
    Ok(())
}
