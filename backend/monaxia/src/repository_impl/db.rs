mod domain;
mod migration;
mod user;

use anyhow::Result;
use monaxia_data::config::Config;
use monaxia_db::establish_pool;
use monaxia_repository::Container;

use std::sync::Arc;

pub async fn construct_container(config: &Config) -> Result<Container> {
    let pool = establish_pool(&config.database.url).await?;

    Ok(Container {
        migration: Arc::new(migration::MigrationRepositoryImpl(pool.clone())),
        user: Arc::new(user::UserRepositoryImpl(pool.clone())),
        domain: Arc::new(domain::DomainpositoryImpl(pool)),
    })
}
