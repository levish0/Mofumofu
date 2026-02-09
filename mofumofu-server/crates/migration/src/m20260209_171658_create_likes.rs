use crate::common::LikeTargetType;
use crate::m20250825_033639_users::Users;
use sea_orm_migration::prelude::*;
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
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Likes::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Likes::TargetType)
                            .enumeration(
                                LikeTargetType::Table,
                                LikeTargetType::iter()
                                    .filter(|v| !matches!(v, LikeTargetType::Table))
                                    .collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Likes::TargetId).uuid().not_null())
                    .col(
                        ColumnDef::new(Likes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_likes_user")
                            .from(Likes::Table, Likes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique: one like per user per target
        manager
            .create_index(
                Index::create()
                    .name("uq_likes_user_target")
                    .table(Likes::Table)
                    .col(Likes::UserId)
                    .col(Likes::TargetType)
                    .col(Likes::TargetId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index: Target's likes
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_target")
                    .table(Likes::Table)
                    .col(Likes::TargetType)
                    .col(Likes::TargetId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Likes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Likes {
    Table,
    Id,
    UserId,
    TargetType,
    TargetId,
    CreatedAt,
}
