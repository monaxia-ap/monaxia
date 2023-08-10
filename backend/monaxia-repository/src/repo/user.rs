use super::Repository;
use crate::RepoResult;

use async_trait::async_trait;
use monaxia_data::user::{LocalUser, LocalUserRegistration, RemoteUserRegistration};

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

    /// Finds a local user by username.
    async fn find_local_user(&self, user_find: UserFind<'_>) -> RepoResult<Option<LocalUser>>;
}

#[derive(Debug, Clone, Copy)]
pub enum UserFind<'a> {
    Username(&'a str),
    UserId(&'a str),
}
