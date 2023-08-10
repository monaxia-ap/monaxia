use super::schema::{Migration, MigrationDef};

use sea_query::{
    ColumnDef, Index, IndexOrder, Order, PostgresQueryBuilder as QueryBuilder, Query, Table,
};
use sea_query_binder::SqlxBinder;
use sqlx::{PgConnection as Connection, Result as SqlxResult};
use time::OffsetDateTime;

pub async fn ensure_migrations_table(conn: &mut Connection) -> SqlxResult<()> {
    let query = Table::create()
        .if_not_exists()
        .table(MigrationDef::Table)
        .col(
            ColumnDef::new(MigrationDef::Id)
                .big_integer()
                .not_null()
                .primary_key()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(MigrationDef::LastMigration)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(MigrationDef::ExecutedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(QueryBuilder);
    sqlx::query(&query).execute(&mut *conn).await?;

    let index_query = Index::create()
        .if_not_exists()
        .name("migrations_execution")
        .table(MigrationDef::Table)
        .col((MigrationDef::ExecutedAt, IndexOrder::Desc))
        .build(QueryBuilder);
    sqlx::query(&index_query).execute(&mut *conn).await?;

    Ok(())
}

pub async fn fetch_last_migration(conn: &mut Connection) -> SqlxResult<Option<Migration>> {
    let (query, values) = Query::select()
        .columns([
            MigrationDef::Id,
            MigrationDef::LastMigration,
            MigrationDef::ExecutedAt,
        ])
        .from(MigrationDef::Table)
        .order_by(MigrationDef::ExecutedAt, Order::Desc)
        .limit(1)
        .build_sqlx(QueryBuilder);
    let row = sqlx::query_as_with(&query, values)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(row)
}

pub async fn register_migration(
    conn: &mut Connection,
    latest_migration_datetime: OffsetDateTime,
    execution_datetime: OffsetDateTime,
) -> SqlxResult<Migration> {
    let (query, values) = Query::insert()
        .into_table(MigrationDef::Table)
        .columns([MigrationDef::LastMigration, MigrationDef::ExecutedAt])
        .values([latest_migration_datetime.into(), execution_datetime.into()])
        .expect("failed to encode")
        .returning_all()
        .build_sqlx(QueryBuilder);
    let row = sqlx::query_as_with(&query, values)
        .fetch_one(&mut *conn)
        .await?;

    Ok(row)
}
