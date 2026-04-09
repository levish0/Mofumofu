use crate::common::notification::NotificationType;
use crate::m20250825_033639_users::Users;
use sea_orm_migration::{prelude::*, schema::*};
use strum::IntoEnumIterator;
use crate::m20250825_033642_create_posts::Posts;
use crate::m20260209_171658_create_comments::Comments;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserNotifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserNotifications::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(UserNotifications::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserNotifications::ActorId).uuid().null())
                    .col(
                        ColumnDef::new(UserNotifications::NotificationType)
                            .enumeration(
                                NotificationType::Table,
                                NotificationType::iter()
                                    .filter(|p| !matches!(p, NotificationType::Table))
                                    .collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserNotifications::Action).text().not_null())
                    // Nullable foreign keys (exactly one must be non-null)
                    .col(ColumnDef::new(UserNotifications::PostId).uuid().null())
                    .col(
                        ColumnDef::new(UserNotifications::CommentId)
                            .uuid()
                            .null(),
                    )
                    // Additional data (optional)
                    .col(
                        ColumnDef::new(UserNotifications::AdditionalData)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserNotifications::IsRead)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        timestamp_with_time_zone(UserNotifications::CreatedAt)
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(UserNotifications::ReadAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    // Foreign keys
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_notifications_user_id")
                            .from(UserNotifications::Table, UserNotifications::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_notifications_actor_id")
                            .from(UserNotifications::Table, UserNotifications::ActorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_notifications_post_id")
                            .from(UserNotifications::Table, UserNotifications::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_notifications_comment_id")
                            .from(UserNotifications::Table, UserNotifications::CommentId)
                            .to(Comments::Table, Comments::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_notifications_post_id")
                            .from(UserNotifications::Table, UserNotifications::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for user's unread notification (most common query)
        // Note: Using ID instead of created_at since UUID v7 is time-sortable
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_user_unread")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::UserId)
                    .col(UserNotifications::IsRead)
                    .col(UserNotifications::Id)
                    .to_owned(),
            )
            .await?;

        // Index for user's all notifications by ID (time-sortable with UUID v7)
        // Supports: WHERE user_id = ? AND id < ? ORDER BY id DESC
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_user_id")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::UserId)
                    .col(UserNotifications::Id)
                    .to_owned(),
            )
            .await?;

        // Index for retention cleanup by created_at
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_created_at")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Index for finding notification by document
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_post")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::PostId)
                    .to_owned(),
            )
            .await?;

        // Index for finding notification by discussion
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_comment")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::CommentId)
                    .to_owned(),
            )
            .await?;

        // Index for notification type analytics
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_type")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::NotificationType)
                    .to_owned(),
            )
            .await?;

        // Index for action filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_user_notifications_action")
                    .table(UserNotifications::Table)
                    .col(UserNotifications::Action)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserNotifications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserNotifications {
    Table,
    Id,
    UserId,
    ActorId,
    NotificationType,
    Action,
    PostId,
    CommentId,
    AdditionalData,
    IsRead,
    CreatedAt,
    ReadAt,
}