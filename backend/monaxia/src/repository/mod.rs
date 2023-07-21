pub mod error;
mod impl_db;

pub use self::error::{Error as RepoError, Result as RepoResult};
pub use self::impl_db::construct_container as construct_container_db;

use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use monaxia_data::migration::Migration;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct Container {
    pub migration: Arc<dyn MigrationRepository>,
    pub user: Arc<dyn UserRepository>,
}

pub trait Repository: Send + Sync + 'static {}

#[async_trait]
pub trait MigrationRepository: Repository {
    async fn ensure_table(&self) -> RepoResult<()>;
    async fn fetch_last_migration(&self) -> RepoResult<Option<Migration>>;
    async fn run_migrations(
        &self,
        migrations: &[(OffsetDateTime, PathBuf)],
    ) -> RepoResult<Option<OffsetDateTime>>;
    async fn register_migration(
        &self,
        last_migration: OffsetDateTime,
        now: OffsetDateTime,
    ) -> RepoResult<Migration>;
}

#[async_trait]
pub trait UserRepository: Repository {
    async fn local_users_count(&self) -> RepoResult<usize>;
}

// for tests

#[cfg(test)]
mod impl_test;
#[cfg(test)]
pub use self::impl_test::construct_container as construct_container_test;
