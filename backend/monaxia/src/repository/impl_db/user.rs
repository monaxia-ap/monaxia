use crate::repository::{RepoResult, Repository, UserRepository};

use async_trait::async_trait;
use monaxia_db::user::action::fetch_local_users_count;
use sqlx::PgPool as Pool;

pub struct UserRepositoryImpl(pub Pool);

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        let count = fetch_local_users_count(&self.0).await?;
        Ok(count)
    }
}
