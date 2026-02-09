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
                    .table(Follows::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Follows::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Follows::FollowerId).uuid().not_null())
                    .col(ColumnDef::new(Follows::FolloweeId).uuid().not_null())
                    .col(
                        ColumnDef::new(Follows::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_follows_follower")
                            .from(Follows::Table, Follows::FollowerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_follows_followee")
                            .from(Follows::Table, Follows::FolloweeId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique: follower + followee
        manager
            .create_index(
                Index::create()
                    .name("uq_follows_follower_followee")
                    .table(Follows::Table)
                    .col(Follows::FollowerId)
                    .col(Follows::FolloweeId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index: Followee's followers
        manager
            .create_index(
                Index::create()
                    .name("idx_follows_followee_id")
                    .table(Follows::Table)
                    .col(Follows::FolloweeId)
                    .to_owned(),
            )
            .await?;

        // CHECK: cannot follow yourself
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE follows ADD CONSTRAINT chk_follows_no_self_follow \
                 CHECK (follower_id != followee_id)",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Follows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Follows {
    Table,
    Id,
    FollowerId,
    FolloweeId,
    CreatedAt,
}
