use crate::m20250825_033639_users::Users;
use crate::m20250825_033643_create_posts::Posts;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Nested replies are modeled only with parent_id.
        // Reply depth is derived at read time instead of being stored.
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
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Comments::PostId).uuid().not_null())
                    .col(ColumnDef::new(Comments::UserId).uuid().not_null())
                    .col(ColumnDef::new(Comments::ParentId).uuid().null())
                    .col(ColumnDef::new(Comments::Content).text().not_null())
                    .col(
                        ColumnDef::new(Comments::LikeCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Comments::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Comments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_post")
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_user")
                            .from(Comments::Table, Comments::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_comments_post_id_id")
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .col(Comments::Id)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_user_id")
                    .table(Comments::Table)
                    .col(Comments::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_post_parent_id_id")
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .col(Comments::ParentId)
                    .col(Comments::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                // Add the same-post self FK after uq_comments_post_id_id exists.
                // Postgres requires the referenced columns to already be unique.
                ForeignKey::create()
                    .name("fk_comments_parent_same_post")
                    .from_tbl(Comments::Table)
                    .from_col(Comments::PostId)
                    .from_col(Comments::ParentId)
                    .to_tbl(Comments::Table)
                    .to_col(Comments::PostId)
                    .to_col(Comments::Id)
                    // Keep deleted parent comments as placeholders when replies still exist.
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Comments {
    Table,
    Id,
    PostId,
    UserId,
    ParentId,
    Content,
    LikeCount,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
