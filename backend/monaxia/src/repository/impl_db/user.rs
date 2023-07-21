use crate::repository::{Repository, UserRepository};

use sqlx::PgPool as Pool;

pub struct UserRepositoryImpl(pub Pool);

impl Repository for UserRepositoryImpl {}
impl UserRepository for UserRepositoryImpl {
    fn local_users_count(&self) -> usize {
        1048576
    }
}
