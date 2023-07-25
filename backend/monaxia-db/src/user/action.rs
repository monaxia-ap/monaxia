use super::schema::{LocalUserDef, UserDef};

use sea_query::{Expr, Func, JoinType, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Acquire, Postgres as DB, Result as SqlxResult};

pub async fn fetch_local_users_count<'a, A: Acquire<'a, Database = DB>>(
    conn: A,
) -> SqlxResult<usize> {
    let mut conn = conn.acquire().await?;
    let (query, _) = Query::select()
        .expr(Func::count(Expr::col(LocalUserDef::UserId)))
        .from(LocalUserDef::Table)
        .build_sqlx(QueryBuilder);
    let (value,): (i64,) = sqlx::query_as(&query).fetch_one(&mut *conn).await?;

    Ok(value as usize)
}

pub async fn local_user_occupied<'a, A: Acquire<'a, Database = DB>>(
    conn: A,
    username: &str,
) -> SqlxResult<bool> {
    let mut conn = conn.acquire().await?;
    let (query, values) = Query::select()
        .column((UserDef::Table, UserDef::Id))
        .from(LocalUserDef::Table)
        .join(
            JoinType::InnerJoin,
            UserDef::Table,
            Expr::col((LocalUserDef::Table, LocalUserDef::UserId))
                .equals((UserDef::Table, UserDef::Id)),
        )
        .cond_where(Expr::col((UserDef::Table, UserDef::Username)).eq(username))
        .build_sqlx(QueryBuilder);

    let occupied: Option<(String,)> = sqlx::query_as_with(&query, values)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(occupied.is_some())
}
