pub mod migration {
    pub mod action;
    pub mod schema;
}

use sqlx::{PgPool as Pool, Result as SqlxResult};

pub async fn establish_pool(database_url: &str) -> SqlxResult<Pool> {
    let pool = Pool::connect(database_url).await?;
    Ok(pool)
}
