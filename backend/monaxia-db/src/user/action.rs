use super::schema::{LocalUser, LocalUserDef, LocalUserInsertion, UserDef, UserInsertion};

use sea_query::{Expr, Func, JoinType, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{PgConnection as Connection, Result as SqlxResult};

pub async fn fetch_local_users_count(conn: &mut Connection) -> SqlxResult<usize> {
    let (query, _) = Query::select()
        .expr(Func::count(Expr::col(LocalUserDef::UserId)))
        .from(LocalUserDef::Table)
        .build_sqlx(QueryBuilder);
    let (value,): (i64,) = sqlx::query_as(&query).fetch_one(&mut *conn).await?;

    Ok(value as usize)
}

pub async fn local_user_occupied(conn: &mut Connection, username: &str) -> SqlxResult<bool> {
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

pub async fn register_user(conn: &mut Connection, insertion: UserInsertion) -> SqlxResult<()> {
    let (query, values) = Query::insert()
        .into_table(UserDef::Table)
        .columns([
            UserDef::Id,
            UserDef::Username,
            UserDef::Domain,
            UserDef::PublicKey,
        ])
        .values([
            insertion.id.into(),
            insertion.username.into(),
            insertion.domain.into(),
            insertion.public_key.into(),
        ])
        .expect("failed to encode")
        .build_sqlx(QueryBuilder);

    sqlx::query_with(&query, values).execute(&mut *conn).await?;
    Ok(())
}

pub async fn register_local_user<'a>(
    conn: &'a mut Connection,
    insertion: LocalUserInsertion<'a>,
) -> SqlxResult<()> {
    let (query, values) = Query::insert()
        .into_table(LocalUserDef::Table)
        .columns([LocalUserDef::UserId, LocalUserDef::PrivateKey])
        .values([insertion.user_id.into(), insertion.private_key.into()])
        .expect("failed to encode")
        .build_sqlx(QueryBuilder);

    sqlx::query_with(&query, values).execute(&mut *conn).await?;
    Ok(())
}

pub async fn find_local_user(
    conn: &mut Connection,
    username: &str,
) -> SqlxResult<Option<LocalUser>> {
    let (query, values) = Query::select()
        .expr_as(Expr::col((UserDef::Table, UserDef::Id)), UserDef::Id)
        .expr_as(Expr::col((UserDef::Table, UserDef::IdSeq)), UserDef::IdSeq)
        .expr_as(
            Expr::col((UserDef::Table, UserDef::Username)),
            UserDef::Username,
        )
        .from(LocalUserDef::Table)
        .join(
            JoinType::InnerJoin,
            UserDef::Table,
            Expr::col((LocalUserDef::Table, LocalUserDef::UserId))
                .equals((UserDef::Table, UserDef::Id)),
        )
        .cond_where(Expr::col((UserDef::Table, UserDef::Username)).eq(username))
        .build_sqlx(QueryBuilder);

    let row = sqlx::query_as_with(&query, values)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(row)
}
