use crate::repository::{RepoResult, Repository, UserRepository};

use async_trait::async_trait;
use sqlx::PgPool as Pool;

pub struct UserRepositoryImpl(pub Pool);

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        Ok(1048576)
    }
}
