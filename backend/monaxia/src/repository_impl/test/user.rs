use async_trait::async_trait;
use monaxia_data::user::{
    LocalUser, LocalUserRegistration, RemoteUserRegistration, User, UserPublicKey,
};
use monaxia_repository::{
    repo::{
        user::{UserFind, UserRepository},
        Repository,
    },
    RepoResult,
};

pub struct UserRepositoryImpl;

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        Ok(1048576)
    }

    async fn local_user_occupied(&self, _username: &str) -> RepoResult<bool> {
        Ok(false)
    }

    async fn register_local_user(
        &self,
        _registration: LocalUserRegistration,
        _domain: &str,
    ) -> RepoResult<User> {
        Ok(test_user())
    }

    async fn register_remote_user(
        &self,
        _registration: RemoteUserRegistration,
        _domain: &str,
    ) -> RepoResult<User> {
        Ok(test_user())
    }

    async fn find_user(&self, _user_find: UserFind<'_>) -> RepoResult<Option<User>> {
        Ok(None)
    }

    async fn find_local_user(&self, _user_find: UserFind<'_>) -> RepoResult<Option<LocalUser>> {
        Ok(None)
    }
}

fn test_user() -> User {
    User {
        id: "12345678".into(),
        id_seq: "1".into(),
        username: "test".into(),
        domain: "example.com".into(),
        public_key: UserPublicKey {
            key_id: "https://example.com/users/test#main-key".into(),
            key_pem: "".into(),
        },
    }
}
