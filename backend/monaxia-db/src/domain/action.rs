use super::schema::DomainDef;

use sea_query::{OnConflict, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{PgPool as Pool, Result as SqlxResult};

pub async fn register_domain(pool: &Pool, domain: &str) -> SqlxResult<bool> {
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

    let (inserted,): (i64,) = sqlx::query_as_with(&query, values).fetch_one(pool).await?;
    Ok(inserted == 1)
}
