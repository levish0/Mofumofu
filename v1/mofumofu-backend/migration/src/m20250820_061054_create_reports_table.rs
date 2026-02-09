use crate::common::{ReportStatus, ReportTargetType};
use sea_orm_migration::{prelude::*, schema::*};
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
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Reports::ReporterId).uuid().null())
                    .col(
                        ColumnDef::new(Reports::TargetType)
                            .enumeration(ReportTargetType::Table, ReportTargetType::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reports::TargetId).uuid().not_null())
                    .col(ColumnDef::new(Reports::Reasons).json().not_null())
                    .col(ColumnDef::new(Reports::Description).text().null())
                    .col(
                        ColumnDef::new(Reports::Status)
                            .enumeration(ReportStatus::Table, ReportStatus::iter().skip(1))
                            .not_null()
                            .default("pending"),
                    )
                    .col(text_null(Reports::AdminNote))
                    .col(uuid_null(Reports::ResolvedBy))
                    .col(timestamp_with_time_zone_null(Reports::ResolvedAt))
                    .col(
                        timestamp_with_time_zone(Reports::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Reports::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_reporter_id")
                            .from(Reports::Table, Reports::ReporterId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
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

        manager
            .create_index(
                Index::create()
                    .name("idx_reports_reporter_created")
                    .table(Reports::Table)
                    .col(Reports::ReporterId)
                    .col(Reports::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_reports_status_created")
                    .table(Reports::Table)
                    .col(Reports::Status)
                    .col(Reports::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reports {
    Table,
    Id,
    ReporterId,
    TargetType,
    TargetId,
    Reasons,
    Description,
    Status,
    AdminNote,
    ResolvedBy,
    ResolvedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
