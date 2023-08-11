use crate::repository_impl::construct_container_db;

use std::{
    env::{current_dir, var as env_var, VarError},
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

use anyhow::{bail, Context, Result};
use clap::Parser;
use monaxia_data::config::Config;
use once_cell::sync::Lazy;
use time::{
    format_description::FormatItem, macros::format_description, OffsetDateTime, PrimitiveDateTime,
};
use tokio::fs::{create_dir_all, read_dir, write};
use tokio_stream::{wrappers::ReadDirStream, StreamExt};
use tracing::{error, info, warn};

static MIGRATION_TIMESTAMP_FORMAT: Lazy<&[FormatItem]> =
    Lazy::new(|| format_description!("[year][month][day][hour][minute][second]"));

#[derive(Debug, Clone, Parser)]
pub struct MigrateSubcommand {
    #[clap(subcommand)]
    command: Option<MxCommand>,
}

#[derive(Debug, Clone, Parser)]
pub enum MxCommand {
    /// Create new migration file.
    New { name: String },
}

pub async fn execute_migrate_subcommand(
    config: Arc<Config>,
    subcommand: MigrateSubcommand,
) -> Result<()> {
    match subcommand.command {
        None => {
            execute_migration(&config).await?;
        }
        Some(MxCommand::New { name }) => {
            create_new_migration(&name).await?;
        }
    }
    Ok(())
}

async fn create_new_migration(name: &str) -> Result<()> {
    let migrations_dir = get_migrations_dir()?;
    let now = OffsetDateTime::now_utc();
    let dt_str = now
        .format(&MIGRATION_TIMESTAMP_FORMAT)
        .expect("invalid datetime format");
    let filename = format!("{}-{}.sql", dt_str, name);

    create_dir_all(&migrations_dir).await?;
    write(migrations_dir.join(&filename), "").await?;
    println!("Created migration file {filename}");

    Ok(())
}

async fn execute_migration(config: &Config) -> Result<()> {
    info!("executing migration...");

    let container = construct_container_db(config).await?;

    container.migration.ensure_table().await?;

    let now = OffsetDateTime::now_utc();
    let last_migration = container.migration.fetch_last_migration().await?;
    let last_migrated = last_migration
        .as_ref()
        .map(|m| m.last_migration)
        .unwrap_or(OffsetDateTime::UNIX_EPOCH);
    let last_executed = last_migration
        .as_ref()
        .map(|m| m.executed_at)
        .unwrap_or(OffsetDateTime::UNIX_EPOCH);

    if last_executed >= now {
        error!("current time is {now}, but last migration executed at {last_executed}");
        bail!("invalid migration");
    }

    let migration_files = get_migrations_file(last_migrated).await?;
    info!("executing {} migration(s)", migration_files.len());

    let last = container.migration.run_migrations(&migration_files).await?;
    if let Some(new_migrated) = last {
        container
            .migration
            .register_migration(new_migrated, now)
            .await?;
    }

    Ok(())
}

fn get_migrations_dir() -> Result<PathBuf> {
    match env_var("CARGO") {
        Ok(cargo) => {
            let workspace_file = String::from_utf8(
                Command::new(cargo)
                    .args(["locate-project", "--workspace", "--message-format=plain"])
                    .output()?
                    .stdout,
            )?;
            let migrations_dir = Path::new(workspace_file.trim())
                .parent()
                .context("invalid workspace root")?
                .join("backend/migrations");
            Ok(migrations_dir)
        }
        Err(VarError::NotPresent) => {
            let curdir = current_dir()?;
            let migrations_dir = curdir.join("migrations");
            Ok(migrations_dir)
        }
        Err(_) => bail!("failed to retrieve Cargo info"),
    }
}

async fn get_migrations_file(
    filter_after: OffsetDateTime,
) -> Result<Vec<(OffsetDateTime, PathBuf)>> {
    let migrations_dir = get_migrations_dir()?;
    let mut migration_stream = ReadDirStream::new(read_dir(&migrations_dir).await?);

    let mut paths = vec![];
    while let Some(de) = migration_stream.next().await {
        let de = de?;
        let ft = de.file_type().await?;
        if !ft.is_file() {
            continue;
        }

        let path = de.path();
        let ext = path.extension().and_then(|t| t.to_str());
        if ext != Some("sql") {
            continue;
        }

        let Some(filename) = path.file_name() else {
            warn!("failed to extract filename from {path:?}, skipping");
            continue;
        };
        let Some(filename_dt) = filename.to_str().filter(|s| s.len() >= 20).map(|s| &s[..14]) else {
            warn!("migration file {path:?} has incorrect filename format, skipping");
            continue;
        };
        let Ok(datetime) = PrimitiveDateTime::parse(filename_dt, &MIGRATION_TIMESTAMP_FORMAT) else {
            warn!("migration file timestamp has incorrect format, skipping");
            continue;
        };
        let datetime = datetime.assume_utc();
        if datetime <= filter_after {
            continue;
        }

        paths.push((datetime, path));
    }
    paths.sort_by_key(|(dt, _)| *dt);

    Ok(paths)
}
