use std::path::PathBuf;

use async_trait::async_trait;
use monaxia_data::migration::Migration;
use monaxia_db::migration::action::{
    ensure_migrations_table, fetch_last_migration, register_migration,
};
use monaxia_repository::{
    repo::{migration::MigrationRepository, Repository},
    RepoResult,
};
use sqlx::{Executor, PgPool as Pool};
use time::OffsetDateTime;
use tokio::fs::read_to_string;
use tracing::info;

pub struct MigrationRepositoryImpl(pub Pool);

impl Repository for MigrationRepositoryImpl {}

#[async_trait]
impl MigrationRepository for MigrationRepositoryImpl {
    async fn ensure_table(&self) -> RepoResult<()> {
        let mut conn = self.0.acquire().await?;
        ensure_migrations_table(&mut conn).await?;
        Ok(())
    }

    async fn fetch_last_migration(&self) -> RepoResult<Option<Migration>> {
        let mut conn = self.0.acquire().await?;
        let migration = fetch_last_migration(&mut conn).await?;
        Ok(migration)
    }

    async fn run_migrations(
        &self,
        migrations: &[(OffsetDateTime, PathBuf)],
    ) -> RepoResult<Option<OffsetDateTime>> {
        let mut last = None;

        let mut tx = self.0.begin().await?;
        for (target_datetime, target_path) in migrations {
            info!("==> {target_path:?}",);
            let sql = read_to_string(target_path).await?;
            tx.execute(&*sql).await?;
            last = Some(*target_datetime);
        }
        tx.commit().await?;

        Ok(last)
    }

    async fn register_migration(
        &self,
        last_migration: OffsetDateTime,
        now: OffsetDateTime,
    ) -> RepoResult<Migration> {
        let mut conn = self.0.acquire().await?;
        let new_migration = register_migration(&mut conn, last_migration, now).await?;
        Ok(new_migration)
    }
}
