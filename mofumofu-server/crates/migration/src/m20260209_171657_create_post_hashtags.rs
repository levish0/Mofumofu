use crate::m20250825_033642_create_posts::Posts;
use crate::m20260209_171633_create_hashtags::Hashtags;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostHashtags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostHashtags::PostId).uuid().not_null())
                    .col(ColumnDef::new(PostHashtags::HashtagId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(PostHashtags::PostId)
                            .col(PostHashtags::HashtagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_hashtags_post")
                            .from(PostHashtags::Table, PostHashtags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_hashtags_hashtag")
                            .from(PostHashtags::Table, PostHashtags::HashtagId)
                            .to(Hashtags::Table, Hashtags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index: Hashtag's posts (reverse lookup)
        manager
            .create_index(
                Index::create()
                    .name("idx_post_hashtags_hashtag_id")
                    .table(PostHashtags::Table)
                    .col(PostHashtags::HashtagId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostHashtags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PostHashtags {
    #[sea_orm(iden = "post_hashtags")]
    Table,
    PostId,
    HashtagId,
}
