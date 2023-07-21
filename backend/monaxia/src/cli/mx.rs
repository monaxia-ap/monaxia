use super::CommonOptions;
use crate::repository::Container;

use std::{
    env::var as env_var,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};
use clap::Parser;
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
pub struct MxSubcommand {
    #[clap(subcommand)]
    command: MxCommand,

    #[clap(flatten)]
    options: CommonOptions,
}

#[derive(Debug, Clone, Parser)]
pub enum MxCommand {
    /// Execute DB migration.
    Migrate,

    /// Create new migration file.
    #[clap(name = "migrate:new")]
    MigrateNew { name: String },
}

pub async fn execute_mx_subcommand(container: Container, subcommand: MxSubcommand) -> Result<()> {
    match subcommand.command {
        MxCommand::Migrate => {
            execute_migration(container).await?;
        }
        MxCommand::MigrateNew { name } => {
            create_new_migration(&name).await?;
        }
    }
    Ok(())
}

pub async fn create_new_migration(name: &str) -> Result<()> {
    let migrations_dir = get_migrations_dir()?;
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

pub async fn execute_migration(container: Container) -> Result<()> {
    info!("executing migration...");

    let now = OffsetDateTime::now_local()?;
    let local_offset = now.offset();

    container.migration.ensure_table().await?;

    // local offset change is potentially unsafe manipulation
    let last_migration = container.migration.fetch_last_migration().await?;
    let local_oldest = OffsetDateTime::UNIX_EPOCH.to_offset(local_offset);
    let last_migrated = last_migration
        .as_ref()
        .map(|m| m.last_migration.to_offset(local_offset))
        .unwrap_or(local_oldest);
    let last_executed = last_migration
        .as_ref()
        .map(|m| m.executed_at.to_offset(local_offset))
        .unwrap_or(local_oldest);

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

    Ok(migrations_dir)
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
        let datetime = match PrimitiveDateTime::parse(filename_dt, &MIGRATION_TIMESTAMP_FORMAT) {
            Ok(dt) => dt.assume_offset(filter_after.offset()),
            Err(err) => {
                warn!("migration file timestamp has incorrect format, skipping ({err})");
                continue;
            }
        };
        if datetime <= filter_after {
            continue;
        }

        paths.push((datetime, path));
    }
    paths.sort_by_key(|(dt, _)| *dt);

    Ok(paths)
}
