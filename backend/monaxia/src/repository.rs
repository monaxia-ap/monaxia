pub mod error;
pub mod r#impl;
pub mod r#trait;

pub use self::error::{Error as RepoError, Result as RepoResult};

use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub migration: Arc<dyn r#trait::migration::MigrationRepository>,
    pub user: Arc<dyn r#trait::user::UserRepository>,
    pub domain: Arc<dyn r#trait::domain::DomainRepository>,
}
