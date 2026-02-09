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
                    .table(HashTags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HashTags::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(HashTags::Name)
                            .string_len(100)
                            .not_null()
                            .unique_key(),
                    ) // #smth
                    .col(
                        ColumnDef::new(HashTags::UsageCount)
                            .integer()
                            .not_null()
                            .default(0), // 사용된 횟수
                    )
                    .col(
                        ColumnDef::new(HashTags::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(HashTags::LastUsedAt)
                            .timestamp_with_time_zone()
                            .null(), // 마지막 사용 시간
                    )
                    .to_owned(),
            )
            .await?;

        // 해시태그 이름으로 검색 최적화
        manager
            .create_index(
                Index::create()
                    .name("idx_hashtags_name")
                    .table(HashTags::Table)
                    .col(HashTags::Name)
                    .to_owned(),
            )
            .await?;

        // 인기 해시태그 조회 (사용 횟수 기준)
        manager
            .create_index(
                Index::create()
                    .name("idx_hashtags_usage_count")
                    .table(HashTags::Table)
                    .col(HashTags::UsageCount)
                    .to_owned(),
            )
            .await?;

        // 최근 사용된 해시태그 조회
        manager
            .create_index(
                Index::create()
                    .name("idx_hashtags_last_used")
                    .table(HashTags::Table)
                    .col(HashTags::LastUsedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HashTags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HashTags {
    Table,
    Id,
    Name,
    UsageCount,
    CreatedAt,
    LastUsedAt,
}
