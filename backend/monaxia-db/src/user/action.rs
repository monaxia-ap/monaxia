use super::schema::LocalUserDef;

use sea_query::{Expr, Func, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{PgPool as Pool, Result as SqlxResult};

pub async fn fetch_local_users_count(pool: &Pool) -> SqlxResult<usize> {
    let (query, _) = Query::select()
        .expr(Func::count(Expr::col(LocalUserDef::UserId)))
        .from(LocalUserDef::Table)
        .build_sqlx(QueryBuilder);
    let (value,): (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;

    Ok(value as usize)
}
