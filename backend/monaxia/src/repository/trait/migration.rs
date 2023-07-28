use super::Repository;
use crate::repository::RepoResult;

use std::path::PathBuf;

use async_trait::async_trait;
use monaxia_data::migration::Migration;
use time::OffsetDateTime;

#[async_trait]
pub trait MigrationRepository: Repository {
    /// Ensures that migrations table exists.
    async fn ensure_table(&self) -> RepoResult<()>;

    /// Fetches latest record of migration log.s
    async fn fetch_last_migration(&self) -> RepoResult<Option<Migration>>;

    /// Runs all migration SQLs.
    async fn run_migrations(
        &self,
        migrations: &[(OffsetDateTime, PathBuf)],
    ) -> RepoResult<Option<OffsetDateTime>>;

    /// Registers new migration log.
    async fn register_migration(
        &self,
        last_migration: OffsetDateTime,
        now: OffsetDateTime,
    ) -> RepoResult<Migration>;
}
