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
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .uuid() // UUID 타입, 고유 식별자
                            .not_null()
                            .primary_key() // PK 지정
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Posts::Title).string_len(80).not_null()) // 블로그 제목
                    .col(ColumnDef::new(Posts::ThumbnailImage).text().null())
                    .col(ColumnDef::new(Posts::Summary).string_len(500).null()) // 요약
                    .col(ColumnDef::new(Posts::Content).text().not_null()) // 본문 (길이 제한 없음)
                    .col(ColumnDef::new(Posts::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Posts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
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
                            .default(0), // 댓글 수
                    )
                    .col(
                        ColumnDef::new(Posts::ViewCount)
                            .integer()
                            .not_null()
                            .default(0), // 조회수
                    )
                    .col(ColumnDef::new(Posts::Slug).string_len(80).not_null()) // URL 슬러그
                    .col(ColumnDef::new(Posts::Render).text().null()) // 렌더링된 HTML 콘텐츠
                    .col(ColumnDef::new(Posts::Toc).json().null()) // TOC 데이터 (JSON)
                    // 작성자와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 기본 인덱스들
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_user_id")
                    .table(Posts::Table)
                    .col(Posts::UserId)
                    .to_owned(),
            )
            .await?;

        // 슬러그로 포스트 찾기
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_slug")
                    .table(Posts::Table)
                    .col(Posts::Slug)
                    .to_owned(),
            )
            .await?;

        // 생성일 기준 정렬
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // 각 사용자에 대해 슬러그가 고유하도록 보장
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    ThumbnailImage,
    Summary,
    Content,
    UserId,
    CreatedAt,
    UpdatedAt,
    LikeCount,
    CommentCount,
    ViewCount,
    Slug,
    Render,
    Toc,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
