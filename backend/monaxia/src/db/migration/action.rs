use super::schema::MigrationDef;

use sea_query::{ColumnDef, Index, IndexOrder, PostgresQueryBuilder as QueryBuilder, Table};
use sqlx::{PgPool as Pool, Result as SqlxResult};

pub async fn ensure_migrations_table(pool: &Pool) -> SqlxResult<()> {
    let query = Table::create()
        .if_not_exists()
        .table(MigrationDef::Table)
        .col(
            ColumnDef::new(MigrationDef::Id)
                .integer()
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
        .index(
            Index::create()
                .name("migrations_execution")
                .col((MigrationDef::ExecutedAt, IndexOrder::Desc)),
        )
        .to_string(QueryBuilder);
    sqlx::query(&query).execute(pool).await?;

    Ok(())
}
