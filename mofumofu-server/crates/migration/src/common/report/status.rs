use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum ReportStatus {
    #[sea_orm(iden = "report_status")]
    Table,
    #[sea_orm(iden = "pending")]
    Pending,
    #[sea_orm(iden = "reviewing")]
    Reviewing,
    #[sea_orm(iden = "resolved")]
    Resolved,
    #[sea_orm(iden = "dismissed")]
    Dismissed,
}
