pub mod error;
mod impl_db;

pub use self::error::{Error as RepoError, Result as RepoResult};
pub use self::impl_db::construct_container as construct_container_db;

use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use monaxia_data::migration::Migration;
use monaxia_data::user::{LocalUserRegistration, RemoteUserRegistration};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct Container {
    pub migration: Arc<dyn MigrationRepository>,
    pub user: Arc<dyn UserRepository>,
    pub domain: Arc<dyn DomainRepository>,
}

pub trait Repository: Send + Sync + 'static {}

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

#[async_trait]
pub trait UserRepository: Repository {
    /// Counts local users.
    async fn local_users_count(&self) -> RepoResult<usize>;

    /// Checks local username occupation. Returns true if occupied.
    async fn local_user_occupied(&self, username: &str) -> RepoResult<bool>;

    /// Registers new user and returns the ID of the user.
    /// Local domain must be registered before this.
    async fn register_local_user(
        &self,
        registration: LocalUserRegistration,
        domain: &str,
    ) -> RepoResult<String>;

    /// Registers new remote user and returns the ID of the user.
    /// Domain must be registered before this.
    async fn register_remote_user(
        &self,
        registration: RemoteUserRegistration,
        domain: &str,
    ) -> RepoResult<String>;
}

#[async_trait]
pub trait DomainRepository: Repository {
    /// Records the domain as acknowledged. Returns true if it was first acknowledgement.
    async fn acknowledge(&self, domain: &str) -> RepoResult<bool>;
}

// for tests

#[cfg(test)]
mod impl_test;
#[cfg(test)]
pub use self::impl_test::construct_container as construct_container_test;
