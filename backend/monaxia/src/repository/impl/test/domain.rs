use crate::repository::{
    r#trait::{domain::DomainRepository, Repository},
    RepoResult,
};
use async_trait::async_trait;

pub struct DomainpositoryImpl;

impl Repository for DomainpositoryImpl {}

#[async_trait]
impl DomainRepository for DomainpositoryImpl {
    async fn acknowledge(&self, _domain: &str) -> RepoResult<bool> {
        Ok(true)
    }
}
