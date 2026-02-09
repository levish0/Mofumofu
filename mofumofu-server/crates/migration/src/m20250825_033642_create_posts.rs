use crate::m20250825_033639_users::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Posts::UserId).uuid().not_null())
                    .col(ColumnDef::new(Posts::Title).text().not_null())
                    .col(ColumnDef::new(Posts::Slug).text().not_null())
                    .col(ColumnDef::new(Posts::ThumbnailImage).text().null())
                    .col(ColumnDef::new(Posts::Summary).text().null())
                    .col(ColumnDef::new(Posts::Content).text().not_null())
                    .col(ColumnDef::new(Posts::Render).text().null())
                    .col(ColumnDef::new(Posts::Toc).json_binary().null())
                    .col(
                        ColumnDef::new(Posts::LikeCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::CommentCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::ViewCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Posts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_user")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index: User's posts
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_user_id")
                    .table(Posts::Table)
                    .col(Posts::UserId)
                    .to_owned(),
            )
            .await?;

        // Index: Slug lookup
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_slug")
                    .table(Posts::Table)
                    .col(Posts::Slug)
                    .to_owned(),
            )
            .await?;

        // Index: Created at (timeline)
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Unique: user_id + slug
        manager
            .create_index(
                Index::create()
                    .name("uq_posts_user_id_slug")
                    .table(Posts::Table)
                    .col(Posts::UserId)
                    .col(Posts::Slug)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Posts {
    Table,
    Id,
    UserId,
    Title,
    Slug,
    ThumbnailImage,
    Summary,
    Content,
    Render,
    Toc,
    LikeCount,
    CommentCount,
    ViewCount,
    CreatedAt,
    UpdatedAt,
}
