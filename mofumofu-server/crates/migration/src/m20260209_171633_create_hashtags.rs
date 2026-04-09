use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Hashtags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hashtags::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Hashtags::Name).text().not_null())
                    .col(
                        ColumnDef::new(Hashtags::NormalizedName)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Hashtags::PostCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Hashtags::LastUsedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Hashtags::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_hashtags_trending")
                    .table(Hashtags::Table)
                    .col(Hashtags::PostCount)
                    .col(Hashtags::LastUsedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Hashtags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Hashtags {
    Table,
    Id,
    Name,
    NormalizedName,
    PostCount,
    LastUsedAt,
    CreatedAt,
}
