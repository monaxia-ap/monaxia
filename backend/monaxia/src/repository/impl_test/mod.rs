mod domain;
mod migration;
mod user;

use super::Container;

use std::sync::Arc;

pub fn construct_container() -> Container {
    Container {
        migration: Arc::new(migration::MigrationRepositoryImpl),
        user: Arc::new(user::UserRepositoryImpl),
        domain: Arc::new(domain::DomainpositoryImpl),
    }
}
