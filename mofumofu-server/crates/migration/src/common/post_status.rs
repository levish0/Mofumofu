use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum PostStatus {
    #[sea_orm(iden = "post_status")]
    Table,
    #[sea_orm(iden = "draft")]
    Draft,
    #[sea_orm(iden = "published")]
    Published,
}
