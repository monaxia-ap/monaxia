use crate::repository::{RepoResult, Repository, UserRepository};

use async_trait::async_trait;
use monaxia_data::{
    id::now_order58,
    user::{LocalUser, LocalUserRegistration, RemoteUserRegistration},
};
use monaxia_db::user::{
    action::{
        fetch_local_users_count, find_local_user, local_user_occupied, register_local_user,
        register_user,
    },
    schema::{LocalUserInsertion, UserInsertion},
};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use sqlx::{Acquire, PgPool as Pool};

pub struct UserRepositoryImpl(pub Pool);

impl Repository for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn local_users_count(&self) -> RepoResult<usize> {
        let mut conn = self.0.acquire().await?;
        let count = fetch_local_users_count(&mut conn).await?;
        Ok(count)
    }

    async fn local_user_occupied(&self, username: &str) -> RepoResult<bool> {
        let mut conn = self.0.acquire().await?;
        let occupied = local_user_occupied(&mut conn, username).await?;
        Ok(occupied)
    }

    async fn register_local_user(
        &self,
        registration: LocalUserRegistration,
        domain: &str,
    ) -> RepoResult<String> {
        let mut tx = self.0.begin().await?;
        let conn = tx.acquire().await?;

        let id = now_order58();
        let public_key = registration
            .private_key
            .to_public_key()
            .to_public_key_pem(LineEnding::LF)
            .expect("failed to write public key");
        let insertion = UserInsertion {
            id: id.clone(),
            username: registration.username.clone(),
            domain: domain.to_string(),
            public_key,
        };
        register_user(&mut *conn, insertion).await?;

        let private_key = registration
            .private_key
            .to_pkcs8_pem(LineEnding::LF)
            .expect("failed to write private key");
        let local_insertion = LocalUserInsertion {
            user_id: id.clone(),
            private_key: private_key.as_str(),
        };
        register_local_user(&mut *conn, local_insertion).await?;

        tx.commit().await?;

        Ok(id)
    }

    async fn register_remote_user(
        &self,
        registration: RemoteUserRegistration,
        domain: &str,
    ) -> RepoResult<String> {
        let mut conn = self.0.acquire().await?;

        let id = now_order58();
        let public_key = registration
            .public_key
            .to_public_key_pem(LineEnding::LF)
            .expect("failed to write public key");
        let insertion = UserInsertion {
            id: id.clone(),
            username: registration.username.clone(),
            domain: domain.to_string(),
            public_key,
        };
        register_user(&mut conn, insertion).await?;

        todo!();
    }

    async fn find_local_user(&self, username: &str) -> RepoResult<Option<LocalUser>> {
        let mut conn = self.0.acquire().await?;
        let user = find_local_user(&mut conn, username).await?;
        Ok(user.map(|u| LocalUser {
            id: u.id,
            id_seq: u.id_seq.to_string(),
            username: u.username,
        }))
    }
}
