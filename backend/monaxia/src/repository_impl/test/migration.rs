use std::path::PathBuf;

use async_trait::async_trait;
use monaxia_data::migration::Migration;
use monaxia_repository::{
    repo::{migration::MigrationRepository, Repository},
    RepoResult,
};
use time::OffsetDateTime;

pub struct MigrationRepositoryImpl;

impl Repository for MigrationRepositoryImpl {}

#[async_trait]
impl MigrationRepository for MigrationRepositoryImpl {
    async fn ensure_table(&self) -> RepoResult<()> {
        unimplemented!();
    }

    async fn fetch_last_migration(&self) -> RepoResult<Option<Migration>> {
        unimplemented!();
    }

    async fn run_migrations(
        &self,
        _migrations: &[(OffsetDateTime, PathBuf)],
    ) -> RepoResult<Option<OffsetDateTime>> {
        unimplemented!();
    }

    async fn register_migration(
        &self,
        _last_migration: OffsetDateTime,
        _now: OffsetDateTime,
    ) -> RepoResult<Migration> {
        unimplemented!();
    }
}
