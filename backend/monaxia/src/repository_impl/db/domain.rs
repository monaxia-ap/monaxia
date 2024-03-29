use async_trait::async_trait;
use monaxia_db::domain::action::register_domain;
use monaxia_repository::{
    repo::{domain::DomainRepository, Repository},
    RepoResult,
};
use sqlx::PgPool as Pool;

pub struct DomainpositoryImpl(pub Pool);

impl Repository for DomainpositoryImpl {}

#[async_trait]
impl DomainRepository for DomainpositoryImpl {
    async fn acknowledge(&self, domain: &str) -> RepoResult<bool> {
        let mut conn = self.0.acquire().await?;
        let new_register = register_domain(&mut conn, domain).await?;
        Ok(new_register)
    }
}
