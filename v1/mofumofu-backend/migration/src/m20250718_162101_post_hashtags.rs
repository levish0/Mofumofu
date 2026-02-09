use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostHashTags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostHashTags::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(PostHashTags::PostId).uuid().not_null())
                    .col(ColumnDef::new(PostHashTags::HashTagId).uuid().not_null())
                    .col(
                        ColumnDef::new(PostHashTags::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // 포스트와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(PostHashTags::Table, PostHashTags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // 해시태그와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(PostHashTags::Table, PostHashTags::HashTagId)
                            .to(HashTags::Table, HashTags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // 복합 유니크 제약 (같은 포스트-해시태그 조합 방지)
                    .index(
                        Index::create()
                            .name("idx_post_hashtags_unique")
                            .col(PostHashTags::PostId)
                            .col(PostHashTags::HashTagId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        // PostHashTags 테이블 인덱스들
        manager
            .create_index(
                Index::create()
                    .name("idx_post_hashtags_post_id")
                    .table(PostHashTags::Table)
                    .col(PostHashTags::PostId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_post_hashtags_hashtag_id")
                    .table(PostHashTags::Table)
                    .col(PostHashTags::HashTagId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostHashTags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostHashTags {
    Table,
    Id,
    PostId,
    HashTagId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum HashTags {
    Table,
    Id,
}
