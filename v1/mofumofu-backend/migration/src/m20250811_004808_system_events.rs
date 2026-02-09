use crate::common::{ActionType, TargetType};
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
                    .table(SystemEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SystemEvents::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(SystemEvents::UserId).uuid().null()) // 시스템 이벤트의 경우 null 가능
                    .col(
                        ColumnDef::new(SystemEvents::ActionType)
                            .enumeration(ActionType::Table, ActionType::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(SystemEvents::TargetId).uuid().null()) // 대상이 없는 액션도 있음
                    .col(
                        ColumnDef::new(SystemEvents::TargetType)
                            .enumeration(TargetType::Table, TargetType::iter().skip(1))
                            .null(), // "post", "hashtag", "user", "comment" 등
                    )
                    .col(
                        ColumnDef::new(SystemEvents::Metadata).json_binary().null(), // 추가 정보
                    )
                    .col(
                        ColumnDef::new(SystemEvents::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // 외래키 (users 테이블과 연결)
                    .foreign_key(
                        ForeignKey::create()
                            .from(SystemEvents::Table, SystemEvents::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull), // 사용자 삭제 시 NULL로 설정
                    )
                    .to_owned(),
            )
            .await?;

        // 액션 타입별 조회 최적화
        manager
            .create_index(
                Index::create()
                    .name("idx_system_events_action_type")
                    .table(SystemEvents::Table)
                    .col(SystemEvents::ActionType)
                    .to_owned(),
            )
            .await?;

        // 사용자별 액션 조회 최적화
        manager
            .create_index(
                Index::create()
                    .name("idx_system_events_user_id")
                    .table(SystemEvents::Table)
                    .col(SystemEvents::UserId)
                    .to_owned(),
            )
            .await?;

        // 시간별 조회 최적화 (트렌딩 계산용)
        manager
            .create_index(
                Index::create()
                    .name("idx_system_events_created_at")
                    .table(SystemEvents::Table)
                    .col(SystemEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // 대상별 액션 조회 최적화
        manager
            .create_index(
                Index::create()
                    .name("idx_system_events_target")
                    .table(SystemEvents::Table)
                    .col(SystemEvents::TargetType)
                    .col(SystemEvents::TargetId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemEvents {
    Table,
    Id,
    UserId,
    ActionType,
    TargetId,
    TargetType,
    Metadata,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
