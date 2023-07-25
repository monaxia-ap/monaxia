use crate::repository::{DomainRepository, RepoResult, Repository};

use async_trait::async_trait;
use monaxia_db::domain::action::register_domain;
use sqlx::PgPool as Pool;

pub struct DomainpositoryImpl(pub Pool);

impl Repository for DomainpositoryImpl {}

#[async_trait]
impl DomainRepository for DomainpositoryImpl {
    async fn acknowledge(&self, domain: &str) -> RepoResult<bool> {
        let new_register = register_domain(&self.0, domain).await?;
        Ok(new_register)
    }
}
