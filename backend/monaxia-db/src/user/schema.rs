use sea_query::Iden;
use sqlx::FromRow;

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

#[derive(Debug)]
pub struct UserInsertion {
    pub id: String,
    pub username: String,
    pub domain: String,
    pub public_key: String,
}

#[derive(Debug)]
pub struct LocalUserInsertion<'a> {
    pub user_id: String,
    pub private_key: &'a str, // this struct does not clear
}

#[derive(Debug, Clone, FromRow)]
pub struct LocalUser {
    pub id: String,
    pub id_seq: i64,
    pub username: String,
}
