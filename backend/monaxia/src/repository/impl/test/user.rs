use crate::repository::{
    r#trait::{
        user::{UserFind, UserRepository},
        Repository,
    },
    RepoResult,
};

use async_trait::async_trait;
use monaxia_data::user::{LocalUser, LocalUserRegistration, RemoteUserRegistration};

pub struct UserRepositoryImpl;

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        Ok(1048576)
    }

    async fn local_user_occupied(&self, _username: &str) -> RepoResult<bool> {
        Ok(false)
    }

    async fn register_local_user(
        &self,
        _registration: LocalUserRegistration,
        _domain: &str,
    ) -> RepoResult<String> {
        Ok("12345678".into())
    }

    async fn register_remote_user(
        &self,
        _registration: RemoteUserRegistration,
        _domain: &str,
    ) -> RepoResult<String> {
        Ok("12345678".into())
    }

    async fn find_local_user(&self, _user_find: UserFind<'_>) -> RepoResult<Option<LocalUser>> {
        Ok(None)
    }
}
