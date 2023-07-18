use sea_query::Iden;

#[derive(Debug, Clone, Copy, Iden)]
pub enum MigrationDef {
    #[iden = "migrations"]
    Table,
    Id,
    LastMigration,
    ExecutedAt,
}
