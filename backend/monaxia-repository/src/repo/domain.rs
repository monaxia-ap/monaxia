use super::Repository;
use crate::RepoResult;

use async_trait::async_trait;

#[async_trait]
pub trait DomainRepository: Repository {
    /// Records the domain as acknowledged. Returns true if it was first acknowledgement.
    async fn acknowledge(&self, domain: &str) -> RepoResult<bool>;
}
