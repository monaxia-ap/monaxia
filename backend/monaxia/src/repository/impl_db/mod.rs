mod domain;
mod migration;
mod user;

use super::Container;

use std::sync::Arc;

use sqlx::PgPool as Pool;

pub fn construct_container(pool: Pool) -> Container {
    Container {
        migration: Arc::new(migration::MigrationRepositoryImpl(pool.clone())),
        user: Arc::new(user::UserRepositoryImpl(pool.clone())),
        domain: Arc::new(domain::DomainpositoryImpl(pool)),
    }
}
