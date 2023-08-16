use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Migration {
    pub id: i64,
    pub last_migration: OffsetDateTime,
    pub executed_at: OffsetDateTime,
}
