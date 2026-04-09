use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum NotificationType {
    #[sea_orm(iden = "notification_type")]
    Table,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "comment")]
    Comment,
    #[sea_orm(iden = "user")]
    User,
    #[sea_orm(iden = "system")]
    System,
}