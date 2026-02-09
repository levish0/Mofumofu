use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum LikeTargetType {
    #[sea_orm(iden = "like_target_type")]
    Table,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "comment")]
    Comment,
}
