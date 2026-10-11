use crate::common::PostStatus;
use crate::m20250825_033639_users::Users;
use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drafts and published posts share the same table.
        // Public fields are allowed to be NULL while drafting, but must exist once published.
        let publish_constraint = Cond::any()
            .add(
                Cond::all()
                    .add(Expr::col(Posts::Status).eq(Expr::val("draft").as_enum(PostStatus::Table)))
                    .add(Expr::col(Posts::PublishedAt).is_null()),
            )
            .add(
                Cond::all()
                    .add(
                        Expr::col(Posts::Status)
                            .eq(Expr::val("published").as_enum(PostStatus::Table)),
                    )
                    .add(Expr::col(Posts::Title).is_not_null())
                    .add(Expr::col(Posts::Slug).is_not_null())
                    .add(Expr::col(Posts::Content).is_not_null())
                    .add(Expr::col(Posts::PublishedAt).is_not_null()),
            );

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
                    .col(
                        ColumnDef::new(Posts::Status)
                            .enumeration(
                                PostStatus::Table,
                                PostStatus::iter()
                                    .filter(|v| !matches!(v, PostStatus::Table))
                                    .collect::<Vec<_>>(),
                            )
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(Posts::Title).text().null())
                    .col(ColumnDef::new(Posts::Slug).text().null())
                    .col(ColumnDef::new(Posts::ThumbnailImage).text().null())
                    .col(ColumnDef::new(Posts::Summary).text().null())
                    .col(ColumnDef::new(Posts::Content).text().null())
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
                        ColumnDef::new(Posts::PublishedAt)
                            .timestamp_with_time_zone()
                            .null(),
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
                    .check(("chk_posts_status_published_at", publish_constraint))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_user_status_id")
                    .table(Posts::Table)
                    .col(Posts::UserId)
                    .col(Posts::Status)
                    .col(Posts::Id)
                    .to_owned(),
            )
            .await?;

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

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_published_at_id_desc")
                    .table(Posts::Table)
                    .col((Posts::PublishedAt, IndexOrder::Desc))
                    .col((Posts::Id, IndexOrder::Desc))
                    .and_where(
                        Expr::col(Posts::Status)
                            .eq(Expr::val("published").as_enum(PostStatus::Table)),
                    )
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
    Status,
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
    PublishedAt,
    CreatedAt,
    UpdatedAt,
}
