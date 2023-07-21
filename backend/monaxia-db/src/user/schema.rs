use sea_query::Iden;

#[derive(Debug, Clone, Copy, Iden)]
pub enum UserDef {
    #[iden = "users"]
    Table,
    Id,
    IdSeq,
    Username,
    Domain,
    PublicKey,
    DisplayName,
    Description,
}

#[derive(Debug, Clone, Copy, Iden)]
pub enum LocalUserDef {
    #[iden = "local_users"]
    Table,
    UserId,
    PrivateKey,
}
