mod user;

use super::Container;

use std::sync::Arc;

use sqlx::PgPool as Pool;

pub fn construct_container(pool: Pool) -> Container {
    Container {
        user: Arc::new(user::UserRepositoryImpl(pool)),
    }
}
