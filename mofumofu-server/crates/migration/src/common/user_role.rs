use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum UserRole {
    #[sea_orm(iden = "user_role")]
    Table,
    #[sea_orm(iden = "user")]
    User,
    #[sea_orm(iden = "moderator")]
    Moderator,
    #[sea_orm(iden = "admin")]
    Admin,
}
