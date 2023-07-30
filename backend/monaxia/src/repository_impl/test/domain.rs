use async_trait::async_trait;
use monaxia_repository::{
    repo::{domain::DomainRepository, Repository},
    RepoResult,
};

pub struct DomainpositoryImpl;

impl Repository for DomainpositoryImpl {}

#[async_trait]
impl DomainRepository for DomainpositoryImpl {
    async fn acknowledge(&self, _domain: &str) -> RepoResult<bool> {
        Ok(true)
    }
}
