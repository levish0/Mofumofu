use sea_orm::prelude::*;
use uuid::Uuid;

use super::common::{ReportStatus, ReportTargetType};
use super::users::Entity as UsersEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub reporter_id: Uuid,
    #[sea_orm(not_null)]
    pub target_type: ReportTargetType,
    #[sea_orm(not_null)]
    pub target_id: Uuid,
    #[sea_orm(column_type = "Text", not_null)]
    pub reason: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(not_null)]
    pub status: ReportStatus,
    #[sea_orm(nullable)]
    pub resolved_by: Option<Uuid>,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub resolved_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::ReporterId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Reporter,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::ResolvedBy",
        to = "super::users::Column::Id",
        on_delete = "SetNull"
    )]
    Resolver,
}

impl ActiveModelBehavior for ActiveModel {}
