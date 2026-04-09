use crate::m20250825_033639_users::Users;
use crate::m20260209_171658_create_comments::Comments;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommentLikes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommentLikes::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(CommentLikes::UserId).uuid().not_null())
                    .col(ColumnDef::new(CommentLikes::CommentId).uuid().not_null())
                    .col(
                        ColumnDef::new(CommentLikes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_likes_user")
                            .from(CommentLikes::Table, CommentLikes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_likes_comment")
                            .from(CommentLikes::Table, CommentLikes::CommentId)
                            .to(Comments::Table, Comments::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_comment_likes_user_comment")
                    .table(CommentLikes::Table)
                    .col(CommentLikes::UserId)
                    .col(CommentLikes::CommentId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comment_likes_comment_id_id")
                    .table(CommentLikes::Table)
                    .col(CommentLikes::CommentId)
                    .col(CommentLikes::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comment_likes_user_id_id")
                    .table(CommentLikes::Table)
                    .col(CommentLikes::UserId)
                    .col(CommentLikes::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommentLikes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CommentLikes {
    #[sea_orm(iden = "comment_likes")]
    Table,
    Id,
    UserId,
    CommentId,
    CreatedAt,
}
