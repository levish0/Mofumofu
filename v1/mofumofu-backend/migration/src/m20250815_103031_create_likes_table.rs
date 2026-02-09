use crate::common::LikeTargetType;
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
                    .table(Likes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Likes::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Likes::UserId).uuid().not_null())
                    .col(ColumnDef::new(Likes::PostId).uuid().null())
                    .col(ColumnDef::new(Likes::CommentId).uuid().null())
                    .col(
                        ColumnDef::new(Likes::TargetType)
                            .enumeration(LikeTargetType::Table, LikeTargetType::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Likes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Likes::Table, Likes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Likes::Table, Likes::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Likes::Table, Likes::CommentId)
                            .to(Comments::Table, Comments::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // post 좋아요용 유니크 인덱스 (null 값 제외)
        manager
            .create_index(
                Index::create()
                    .name("idx_unique_user_post_like")
                    .table(Likes::Table)
                    .col(Likes::UserId)
                    .col(Likes::PostId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // comment 좋아요용 유니크 인덱스 (null 값 제외)
        manager
            .create_index(
                Index::create()
                    .name("idx_unique_user_comment_like")
                    .table(Likes::Table)
                    .col(Likes::UserId)
                    .col(Likes::CommentId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // user_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_user_id")
                    .table(Likes::Table)
                    .col(Likes::UserId)
                    .to_owned(),
            )
            .await?;

        // post_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_post_id")
                    .table(Likes::Table)
                    .col(Likes::PostId)
                    .to_owned(),
            )
            .await?;

        // comment_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_comment_id")
                    .table(Likes::Table)
                    .col(Likes::CommentId)
                    .to_owned(),
            )
            .await?;

        // target_type 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_target_type")
                    .table(Likes::Table)
                    .col(Likes::TargetType)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Likes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Likes {
    Table,
    Id,
    UserId,
    PostId,
    CommentId,
    TargetType,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
}
