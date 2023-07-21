pub mod impl_db;

use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub user: Arc<dyn UserRepository>,
}

pub trait Repository: Send + Sync + 'static {}

pub trait UserRepository: Repository {
    fn local_users_count(&self) -> usize;
}
