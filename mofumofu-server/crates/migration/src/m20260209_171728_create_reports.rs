use crate::common::report::ReportStatus;
use crate::m20250825_033639_users::Users;
use crate::m20250825_033643_create_posts::Posts;
use crate::m20260209_171658_create_comments::Comments;
use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Reports are a live moderation queue, not the final audit log.
        // Moderator actions are persisted separately in moderation_logs.
        let target_constraint = Cond::any()
            .add(
                Cond::all()
                    .add(Expr::col(Reports::TargetUserId).is_not_null())
                    .add(Expr::col(Reports::TargetPostId).is_null())
                    .add(Expr::col(Reports::TargetCommentId).is_null()),
            )
            .add(
                Cond::all()
                    .add(Expr::col(Reports::TargetUserId).is_null())
                    .add(Expr::col(Reports::TargetPostId).is_not_null())
                    .add(Expr::col(Reports::TargetCommentId).is_null()),
            )
            .add(
                Cond::all()
                    .add(Expr::col(Reports::TargetUserId).is_null())
                    .add(Expr::col(Reports::TargetPostId).is_null())
                    .add(Expr::col(Reports::TargetCommentId).is_not_null()),
            );

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
                    .col(ColumnDef::new(Reports::TargetUserId).uuid().null())
                    .col(ColumnDef::new(Reports::TargetPostId).uuid().null())
                    .col(ColumnDef::new(Reports::TargetCommentId).uuid().null())
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
                            .default("pending"),
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
                    .check(("chk_reports_exactly_one_target", target_constraint))
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_target_user")
                            .from(Reports::Table, Reports::TargetUserId)
                            .to(Users::Table, Users::Id)
                            // If the target disappears by hard delete, the open report is dropped as well.
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_target_post")
                            .from(Reports::Table, Reports::TargetPostId)
                            .to(Posts::Table, Posts::Id)
                            // Reports only track currently existing targets.
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_target_comment")
                            .from(Reports::Table, Reports::TargetCommentId)
                            .to(Comments::Table, Comments::Id)
                            // Reports only track currently existing targets.
                            .on_delete(ForeignKeyAction::Cascade),
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

        manager
            .create_index(
                Index::create()
                    .name("idx_reports_target_user_id")
                    .table(Reports::Table)
                    .col(Reports::TargetUserId)
                    .and_where(Expr::col(Reports::TargetUserId).is_not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_reports_target_post_id")
                    .table(Reports::Table)
                    .col(Reports::TargetPostId)
                    .and_where(Expr::col(Reports::TargetPostId).is_not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_reports_target_comment_id")
                    .table(Reports::Table)
                    .col(Reports::TargetCommentId)
                    .and_where(Expr::col(Reports::TargetCommentId).is_not_null())
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
    TargetUserId,
    TargetPostId,
    TargetCommentId,
    Reason,
    Description,
    Status,
    ResolvedBy,
    ResolvedAt,
    CreatedAt,
}
