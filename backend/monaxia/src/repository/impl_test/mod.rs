mod user;

use super::Container;

use std::sync::Arc;

pub fn construct_container() -> Container {
    Container {
        user: Arc::new(user::UserRepositoryImpl),
    }
}
