use crate::common::report::ReportTargetType;
use crate::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ReportTargetType::Table)
                    .values(
                        ReportTargetType::iter()
                            .filter(|v| !matches!(v, ReportTargetType::Table))
                            .collect::<Vec<_>>(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(ReportTargetType::Table)
                    .to_owned(),
            )
            .await
    }
}
