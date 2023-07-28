use sea_query::Iden;

#[derive(Debug, Clone, Copy, Iden)]
pub enum DomainDef {
    #[iden = "domains"]
    Table,
    Domain,
    RecognizedAt,
}
