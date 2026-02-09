use crate::common::report::{ReportStatus, ReportTargetType};
use crate::m20250825_033639_users::Users;
use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reports::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Reports::ReporterId).uuid().not_null())
                    .col(
                        ColumnDef::new(Reports::TargetType)
                            .enumeration(
                                ReportTargetType::Table,
                                ReportTargetType::iter()
                                    .filter(|v| !matches!(v, ReportTargetType::Table))
                                    .collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reports::TargetId).uuid().not_null())
                    .col(ColumnDef::new(Reports::Reason).text().not_null())
                    .col(ColumnDef::new(Reports::Description).text().null())
                    .col(
                        ColumnDef::new(Reports::Status)
                            .enumeration(
                                ReportStatus::Table,
                                ReportStatus::iter()
                                    .filter(|v| !matches!(v, ReportStatus::Table))
                                    .collect::<Vec<_>>(),
                            )
                            .not_null()
                            .default(Expr::cust("'pending'::report_status")),
                    )
                    .col(ColumnDef::new(Reports::ResolvedBy).uuid().null())
                    .col(
                        ColumnDef::new(Reports::ResolvedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Reports::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_reporter")
                            .from(Reports::Table, Reports::ReporterId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_resolved_by")
                            .from(Reports::Table, Reports::ResolvedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Index: Reporter's reports
        manager
            .create_index(
                Index::create()
                    .name("idx_reports_reporter_id")
                    .table(Reports::Table)
                    .col(Reports::ReporterId)
                    .to_owned(),
            )
            .await?;

        // Index: Target lookup
        manager
            .create_index(
                Index::create()
                    .name("idx_reports_target")
                    .table(Reports::Table)
                    .col(Reports::TargetType)
                    .col(Reports::TargetId)
                    .to_owned(),
            )
            .await?;

        // Index: Status filter
        manager
            .create_index(
                Index::create()
                    .name("idx_reports_status")
                    .table(Reports::Table)
                    .col(Reports::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Reports {
    Table,
    Id,
    ReporterId,
    TargetType,
    TargetId,
    Reason,
    Description,
    Status,
    ResolvedBy,
    ResolvedAt,
    CreatedAt,
}
