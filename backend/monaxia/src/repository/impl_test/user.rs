use crate::repository::{RepoResult, Repository, UserRepository};

use async_trait::async_trait;

pub struct UserRepositoryImpl;

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        Ok(1048576)
    }
}
