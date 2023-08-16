use async_trait::async_trait;
use monaxia_data::{
    id::now_order58,
    user::{
        generate_local_user_url, LocalUser, LocalUserRegistration, LocalUserUrl,
        RemoteUserRegistration, User, UserPublicKey,
    },
};
use monaxia_db::user::{
    action::{
        fetch_local_users_count, find_user_by_column, local_user_occupied, register_local_user,
        register_user,
    },
    schema::{LocalUserInsertion, UserDef, UserInsertion},
};
use monaxia_repository::{
    repo::{
        user::{UserFind, UserRepository},
        Repository,
    },
    RepoResult,
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
    ) -> RepoResult<User> {
        let mut tx = self.0.begin().await?;
        let conn = tx.acquire().await?;

        // register common part
        let id = now_order58();

        let public_key = registration
            .private_key
            .to_public_key()
            .to_public_key_pem(LineEnding::LF)
            .expect("failed to write public key");
        let public_key_id =
            generate_local_user_url(&registration.base_url, &id, LocalUserUrl::KeyId).to_string();
        let insertion = UserInsertion {
            id: id.clone(),
            username: registration.username.clone(),
            domain: domain.to_string(),
            public_key,
            public_key_id,
        };
        let registered_user = register_user(&mut *conn, insertion).await?;

        // local-only part
        let private_key = registration
            .private_key
            .to_pkcs8_pem(LineEnding::LF)
            .expect("failed to write private key");
        let local_insertion = LocalUserInsertion {
            user_id: registered_user.id.clone(),
            private_key: private_key.as_str(),
        };
        register_local_user(&mut *conn, local_insertion).await?;

        tx.commit().await?;

        let user = User {
            id: registered_user.id,
            id_seq: registered_user.id_seq.to_string(),
            username: registered_user.username,
            domain: registered_user.domain,
            public_key: UserPublicKey {
                key_id: registered_user.public_key_id,
                key_pem: registered_user.public_key,
            },
        };
        Ok(user)
    }

    async fn register_remote_user(
        &self,
        registration: RemoteUserRegistration,
        domain: &str,
    ) -> RepoResult<User> {
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
            public_key_id: registration.public_key_id,
        };
        let registered_user = register_user(&mut conn, insertion).await?;

        let user = User {
            id: registered_user.id,
            id_seq: registered_user.id_seq.to_string(),
            username: registered_user.username,
            domain: registered_user.domain,
            public_key: UserPublicKey {
                key_id: registered_user.public_key_id,
                key_pem: registered_user.public_key,
            },
        };
        Ok(user)
    }

    async fn find_user(&self, user_find: UserFind<'_>) -> RepoResult<Option<User>> {
        let mut conn = self.0.acquire().await?;
        let user = match user_find {
            UserFind::Username(un) => {
                find_user_by_column(&mut conn, false, UserDef::Username, un).await?
            }
            UserFind::UserId(id) => find_user_by_column(&mut conn, false, UserDef::Id, id).await?,
            UserFind::KeyId(kid) => {
                find_user_by_column(&mut conn, false, UserDef::PublicKeyId, kid).await?
            }
        };
        Ok(user.map(|u| User {
            id: u.id,
            id_seq: u.id_seq.to_string(),
            username: u.username,
            domain: u.domain,
            public_key: UserPublicKey {
                key_id: u.public_key_id,
                key_pem: u.public_key,
            },
        }))
    }

    async fn find_local_user(&self, user_find: UserFind<'_>) -> RepoResult<Option<LocalUser>> {
        let mut conn = self.0.acquire().await?;
        let user = match user_find {
            UserFind::Username(un) => {
                find_user_by_column(&mut conn, true, UserDef::Username, un).await?
            }
            UserFind::UserId(id) => find_user_by_column(&mut conn, true, UserDef::Id, id).await?,
            UserFind::KeyId(kid) => {
                find_user_by_column(&mut conn, true, UserDef::PublicKeyId, kid).await?
            }
        };
        Ok(user.map(|u| LocalUser {
            id: u.id,
            id_seq: u.id_seq.to_string(),
            username: u.username,
            public_key: UserPublicKey {
                key_id: u.public_key_id,
                key_pem: u.public_key,
            },
        }))
    }
}
