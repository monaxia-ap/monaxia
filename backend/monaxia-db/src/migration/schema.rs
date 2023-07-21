use sea_query::Iden;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, Iden)]
pub enum MigrationDef {
    #[iden = "migrations"]
    Table,
    Id,
    LastMigration,
    ExecutedAt,
}

#[derive(Debug, Clone, FromRow)]
pub struct Migration {
    pub id: i64,
    pub last_migration: OffsetDateTime,
    pub executed_at: OffsetDateTime,
}
