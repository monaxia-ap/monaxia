pub mod migration {
    pub mod action;
    pub mod schema;
}

use std::time::Duration;

use sqlx::{pool::PoolOptions, PgPool as Pool, Result as SqlxResult};

pub async fn establish_pool(database_url: &str) -> SqlxResult<Pool> {
    let options = PoolOptions::new().acquire_timeout(Duration::from_secs(5));
    let pool = options.connect(database_url).await?;
    Ok(pool)
}
