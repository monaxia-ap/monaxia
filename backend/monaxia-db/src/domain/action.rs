use super::schema::DomainDef;

use sea_query::{OnConflict, PostgresQueryBuilder as QueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{PgConnection as Connection, Result as SqlxResult};
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn register_domain(conn: &mut Connection, domain: &str) -> SqlxResult<bool> {
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

    let result = sqlx::query_with(&query, values).execute(&mut *conn).await?;
    Ok(result.rows_affected() == 1)
}
