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
                    .table(Drafts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Drafts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Drafts::UserId).uuid().not_null())
                    .col(ColumnDef::new(Drafts::Title).text().null())
                    .col(ColumnDef::new(Drafts::Slug).text().null())
                    .col(ColumnDef::new(Drafts::Content).text().null())
                    .col(ColumnDef::new(Drafts::Metadata).json_binary().null())
                    .col(
                        ColumnDef::new(Drafts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Drafts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_drafts_user")
                            .from(Drafts::Table, Drafts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index: User's drafts
        manager
            .create_index(
                Index::create()
                    .name("idx_drafts_user_id")
                    .table(Drafts::Table)
                    .col(Drafts::UserId)
                    .to_owned(),
            )
            .await?;

        // Unique: user_id + slug
        manager
            .create_index(
                Index::create()
                    .name("uq_drafts_user_id_slug")
                    .table(Drafts::Table)
                    .col(Drafts::UserId)
                    .col(Drafts::Slug)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Drafts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Drafts {
    Table,
    Id,
    UserId,
    Title,
    Slug,
    Content,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
