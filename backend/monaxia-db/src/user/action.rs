use super::schema::{LocalUser, LocalUserDef, LocalUserInsertion, User, UserDef, UserInsertion};

use sea_query::{Expr, Func, JoinType, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{PgConnection as Connection, Result as SqlxResult};
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn fetch_local_users_count(conn: &mut Connection) -> SqlxResult<usize> {
    let (query, _) = Query::select()
        .expr(Func::count(Expr::col(LocalUserDef::UserId)))
        .from(LocalUserDef::Table)
        .build_sqlx(QueryBuilder);
    let (value,): (i64,) = sqlx::query_as(&query).fetch_one(&mut *conn).await?;

    Ok(value as usize)
}

#[instrument(skip(conn))]
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

#[instrument(skip(conn))]
pub async fn register_user(conn: &mut Connection, insertion: UserInsertion) -> SqlxResult<User> {
    let (query, values) = Query::insert()
        .into_table(UserDef::Table)
        .columns([
            UserDef::Id,
            UserDef::Username,
            UserDef::Domain,
            UserDef::PublicKey,
            UserDef::PublicKeyId,
        ])
        .values([
            insertion.id.into(),
            insertion.username.into(),
            insertion.domain.into(),
            insertion.public_key.into(),
            insertion.public_key_id.into(),
        ])
        .expect("failed to encode")
        .returning_all()
        .build_sqlx(QueryBuilder);

    let row = sqlx::query_as_with(&query, values)
        .fetch_one(&mut *conn)
        .await?;
    Ok(row)
}

#[instrument(skip(conn))]
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

#[instrument(skip(conn))]
pub async fn find_local_user_by_username(
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
        .expr_as(
            Expr::col((UserDef::Table, UserDef::PublicKey)),
            UserDef::PublicKey,
        )
        .expr_as(
            Expr::col((UserDef::Table, UserDef::PublicKeyId)),
            UserDef::PublicKeyId,
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

#[instrument(skip(conn))]
pub async fn find_local_user_by_id(
    conn: &mut Connection,
    user_id: &str,
) -> SqlxResult<Option<LocalUser>> {
    let (query, values) = Query::select()
        .expr_as(Expr::col((UserDef::Table, UserDef::Id)), UserDef::Id)
        .expr_as(Expr::col((UserDef::Table, UserDef::IdSeq)), UserDef::IdSeq)
        .expr_as(
            Expr::col((UserDef::Table, UserDef::Username)),
            UserDef::Username,
        )
        .expr_as(
            Expr::col((UserDef::Table, UserDef::PublicKey)),
            UserDef::PublicKey,
        )
        .expr_as(
            Expr::col((UserDef::Table, UserDef::PublicKeyId)),
            UserDef::PublicKeyId,
        )
        .from(LocalUserDef::Table)
        .join(
            JoinType::InnerJoin,
            UserDef::Table,
            Expr::col((LocalUserDef::Table, LocalUserDef::UserId))
                .equals((UserDef::Table, UserDef::Id)),
        )
        .cond_where(Expr::col((UserDef::Table, UserDef::Id)).eq(user_id))
        .build_sqlx(QueryBuilder);

    let row = sqlx::query_as_with(&query, values)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(row)
}

#[instrument(skip(conn))]
pub async fn find_local_user_by_key_id(
    conn: &mut Connection,
    key_id: &str,
) -> SqlxResult<Option<LocalUser>> {
    let (query, values) = Query::select()
        .expr_as(Expr::col((UserDef::Table, UserDef::Id)), UserDef::Id)
        .expr_as(Expr::col((UserDef::Table, UserDef::IdSeq)), UserDef::IdSeq)
        .expr_as(
            Expr::col((UserDef::Table, UserDef::Username)),
            UserDef::Username,
        )
        .expr_as(
            Expr::col((UserDef::Table, UserDef::PublicKey)),
            UserDef::PublicKey,
        )
        .expr_as(
            Expr::col((UserDef::Table, UserDef::PublicKeyId)),
            UserDef::PublicKeyId,
        )
        .from(LocalUserDef::Table)
        .join(
            JoinType::InnerJoin,
            UserDef::Table,
            Expr::col((LocalUserDef::Table, LocalUserDef::UserId))
                .equals((UserDef::Table, UserDef::Id)),
        )
        .cond_where(Expr::col((UserDef::Table, UserDef::PublicKeyId)).eq(key_id))
        .build_sqlx(QueryBuilder);

    let row = sqlx::query_as_with(&query, values)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(row)
}
