use crate::repository::{RepoResult, Repository, UserRepository};

use async_trait::async_trait;
use monaxia_data::{
    id::now_order58,
    user::{LocalUserRegistration, RemoteUserRegistration},
};
use monaxia_db::user::action::{fetch_local_users_count, local_user_occupied};
use sqlx::PgPool as Pool;

pub struct UserRepositoryImpl(pub Pool);

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        let count = fetch_local_users_count(&self.0).await?;
        Ok(count)
    }

    async fn local_user_occupied(&self, username: &str) -> RepoResult<bool> {
        let occupied = local_user_occupied(&self.0, username).await?;
        Ok(occupied)
    }

    async fn register_local_user(
        &self,
        registration: LocalUserRegistration,
        domain: &str,
    ) -> RepoResult<String> {
        let remote_user = RemoteUserRegistration {
            username: registration.username.clone(),
            public_key: registration.private_key.to_public_key(),
        };
        todo!();
    }

    async fn register_remote_user(
        &self,
        registration: RemoteUserRegistration,
        domain: &str,
    ) -> RepoResult<String> {
        let id = now_order58();

        todo!();
    }
}
