use super::CommonOptions;
use crate::{
    config::read_config,
    db::{establish_pool, migration::action::ensure_migrations_table},
};

use std::{env::var as env_var, path::Path, process::Command};

use anyhow::{Context, Result};
use clap::Parser;
use once_cell::sync::Lazy;
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};
use tokio::fs::{create_dir_all, write};

static MIGRATION_TIMESTAMP_FORMAT: Lazy<&[FormatItem]> =
    Lazy::new(|| format_description!("[year][month][day][hour][minute][second]"));

#[derive(Debug, Clone, Parser)]
pub enum MxSubcommand {
    /// Execute DB migration.
    Migrate,

    /// Create new migration file.
    #[clap(name = "migrate:new")]
    MigrateNew { name: String },
}

pub async fn execute_mx_subcommand(options: CommonOptions, subcommand: MxSubcommand) -> Result<()> {
    match subcommand {
        MxSubcommand::Migrate => {
            execute_migration(options).await?;
        }
        MxSubcommand::MigrateNew { name } => {
            create_new_migration(&name).await?;
        }
    }
    Ok(())
}

pub async fn create_new_migration(name: &str) -> Result<()> {
    let cargo = env_var("CARGO").context("cannot locate cargo")?;
    let workspace_file = String::from_utf8(
        Command::new(cargo)
            .args(["locate-project", "--workspace", "--message-format=plain"])
            .output()?
            .stdout,
    )?;
    let migrations_dir = Path::new(workspace_file.trim())
        .parent()
        .context("invalid workspace root")?
        .join("migrations");

    let now = OffsetDateTime::now_local().expect("cannot fetch local time");
    let dt_str = now
        .format(&MIGRATION_TIMESTAMP_FORMAT)
        .expect("invalid datetime format");
    let filename = format!("{}-{}.sql", dt_str, name);

    create_dir_all(&migrations_dir).await?;
    write(migrations_dir.join(&filename), "").await?;
    println!("Created migration file {filename}");

    Ok(())
}

pub async fn execute_migration(options: CommonOptions) -> Result<()> {
    let config = read_config(&options.config).await?;
    let pool = establish_pool(&config.database.url).await?;

    ensure_migrations_table(&pool).await?;

    Ok(())
}
