use super::schema::DomainDef;

use sea_query::{OnConflict, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Acquire, Postgres as DB, Result as SqlxResult};

pub async fn register_domain<'a, A: Acquire<'a, Database = DB>>(
    conn: A,
    domain: &str,
) -> SqlxResult<bool> {
    let mut conn = conn.acquire().await?;
    let (query, values) = Query::insert()
        .into_table(DomainDef::Table)
        .columns([DomainDef::Domain])
        .values([domain.into()])
        .expect("failed to encode")
        .on_conflict(
            OnConflict::column(DomainDef::Domain)
                .do_nothing()
                .to_owned(),
        )
        .build_sqlx(QueryBuilder);

    let (inserted,): (i64,) = sqlx::query_as_with(&query, values)
        .fetch_one(&mut *conn)
        .await?;
    Ok(inserted == 1)
}
