use crate::m20250825_033639_users::Users;
use crate::m20250825_033643_create_posts::Posts;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostLikes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostLikes::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(PostLikes::UserId).uuid().not_null())
                    .col(ColumnDef::new(PostLikes::PostId).uuid().not_null())
                    .col(
                        ColumnDef::new(PostLikes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_likes_user")
                            .from(PostLikes::Table, PostLikes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_likes_post")
                            .from(PostLikes::Table, PostLikes::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_post_likes_user_post")
                    .table(PostLikes::Table)
                    .col(PostLikes::UserId)
                    .col(PostLikes::PostId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_post_likes_post_id_id")
                    .table(PostLikes::Table)
                    .col(PostLikes::PostId)
                    .col(PostLikes::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_post_likes_user_id_id")
                    .table(PostLikes::Table)
                    .col(PostLikes::UserId)
                    .col(PostLikes::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostLikes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PostLikes {
    #[sea_orm(iden = "post_likes")]
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
}
