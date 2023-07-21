pub mod error;
mod impl_db;

use async_trait::async_trait;

pub use self::error::{Error as RepoError, Result as RepoResult};
pub use self::impl_db::construct_container as construct_container_db;

use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub user: Arc<dyn UserRepository>,
}

pub trait Repository: Send + Sync + 'static {}

#[async_trait]
pub trait UserRepository: Repository {
    async fn local_users_count(&self) -> RepoResult<usize>;
}
