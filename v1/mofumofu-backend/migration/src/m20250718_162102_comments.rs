use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Comments::Content).text().not_null()) // 댓글 내용
                    .col(ColumnDef::new(Comments::PostId).uuid().not_null()) // 포스트 ID
                    .col(ColumnDef::new(Comments::UserId).uuid().not_null()) // 댓글 작성자
                    .col(ColumnDef::new(Comments::ParentId).uuid().null()) // 대댓글용 부모 댓글 ID
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Comments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Comments::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Comments::LikeCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Comments::ReplyCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    // 포스트와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // 댓글 작성자와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // 대댓글 관계 외래키 (자기 참조)
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::ParentId)
                            .to(Comments::Table, Comments::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_post_id")
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .to_owned(),
            )
            .await?;

        // 특정 포스트의 댓글들을 시간순으로 조회
        manager
            .create_index(
                Index::create()
                    .name("idx_comments_post_created")
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .col(Comments::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // 대댓글 조회 최적화
        manager
            .create_index(
                Index::create()
                    .name("idx_comments_parent_id")
                    .table(Comments::Table)
                    .col(Comments::ParentId)
                    .to_owned(),
            )
            .await?;

        // 특정 포스트의 댓글들을 좋아요순으로 조회
        manager
            .create_index(
                Index::create()
                    .name("idx_comments_post_likes")
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .col(Comments::LikeCount)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    Content,
    PostId,
    UserId,
    ParentId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
    LikeCount,
    ReplyCount,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
