use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum ReportTargetType {
    #[sea_orm(iden = "report_target_type")]
    Table,
    #[sea_orm(iden = "user")]
    User,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "comment")]
    Comment,
}
