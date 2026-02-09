use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // user_refresh_tokens 테이블 생성
        // - 각 row는 하나의 refresh token 정보(유저별, 여러 기기/브라우저 지원)
        // - id: UUID, PK, 토큰 row 식별자
        // - user_id: 토큰 소유 유저, users 테이블과 외래키 연결
        // - refresh_token: 실제 토큰 문자열, NOT NULL
        // - expires_at: 만료 시각, NOT NULL (세션 만료/보안)
        // - created_at: 발급 시각, NOT NULL (감사/이력)
        // - revoked_at: 폐기 시각, NULL 허용(폐기 시 기록)
        // - 외래키: user_id → users.id (유저 삭제 시 토큰 자동 삭제, CASCADE)
        manager
            .create_table(
                Table::create()
                    .table(UserRefreshTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRefreshTokens::Id)
                            .uuid() // UUID 타입, 고유 식별자
                            .not_null()
                            .primary_key() // PK 지정
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::UserId)
                            .uuid() // users 테이블의 id와 동일 타입
                            .not_null(), // 반드시 유저가 있어야 함
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::RefreshToken)
                            .text() // 토큰 문자열, 길이 제한 없음
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::ExpiresAt)
                            .timestamp_with_time_zone() // 만료 시각, 타임존 포함
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::CreatedAt)
                            .timestamp_with_time_zone() // 발급 시각
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::RevokedAt)
                            .timestamp_with_time_zone() // 폐기 시각, NULL 허용
                            .null(),
                    )
                    .col(ColumnDef::new(UserRefreshTokens::IpAddress).text().null())
                    .col(ColumnDef::new(UserRefreshTokens::UserAgent).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRefreshTokens::Table, UserRefreshTokens::UserId) // user_id 컬럼 기준
                            .to(Users::Table, Users::Id) // users 테이블 id 참조
                            .on_delete(ForeignKeyAction::Cascade), // 유저 삭제 시 토큰도 자동 삭제
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // user_refresh_tokens 테이블 삭제
        // - 토큰 이력 전체 삭제됨
        manager
            .drop_table(Table::drop().table(UserRefreshTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRefreshTokens {
    Table,
    Id,
    UserId,
    RefreshToken,
    ExpiresAt,
    CreatedAt,
    RevokedAt,
    IpAddress,
    UserAgent,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
