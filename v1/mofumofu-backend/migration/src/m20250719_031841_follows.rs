use sea_orm_migration::{prelude::*, schema::*};

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
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Follows::FollowerId).uuid().not_null())
                    .col(ColumnDef::new(Follows::FolloweeId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Follows::Table, Follows::FollowerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Follows::Table, Follows::FolloweeId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_unique_follower_followee")
                    .table(Follows::Table)
                    .col(Follows::FollowerId)
                    .col(Follows::FolloweeId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // follower_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_follower_id")
                    .table(Follows::Table)
                    .col(Follows::FollowerId)
                    .to_owned(),
            )
            .await?;

        // followee_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_followee_id")
                    .table(Follows::Table)
                    .col(Follows::FolloweeId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Follows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Follows {
    Table,
    Id,
    FollowerId,
    FolloweeId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
