pub mod error;
pub mod repo;

pub use self::error::{Error as RepoError, Result as RepoResult};

use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub migration: Arc<dyn repo::migration::MigrationRepository>,
    pub user: Arc<dyn repo::user::UserRepository>,
    pub domain: Arc<dyn repo::domain::DomainRepository>,
}
