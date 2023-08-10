pub mod domain;
pub mod migration;
pub mod user;

pub trait Repository: Send + Sync + 'static {}
