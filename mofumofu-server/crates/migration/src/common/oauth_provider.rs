use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum OAuthProvider {
    #[sea_orm(iden = "user_oauth_provider")]
    Table,
    #[sea_orm(iden = "google")]
    Google,
    #[sea_orm(iden = "github")]
    Github,
    #[sea_orm(iden = "discord")]
    Discord,
    #[sea_orm(iden = "x")]
    X,
    #[sea_orm(iden = "microsoft")]
    Microsoft,
}
