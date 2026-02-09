use crate::entity::common::{ReportStatus, ReportTargetType};
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub reporter_id: Option<Uuid>,

    pub target_type: ReportTargetType,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub target_id: Uuid,

    #[sea_orm(column_type = "Json")]
    pub reasons: serde_json::Value,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    pub status: ReportStatus,

    #[sea_orm(column_type = "Text", nullable)]
    pub admin_note: Option<String>,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub resolved_by: Option<Uuid>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub resolved_at: Option<DateTimeUtc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::ReporterId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Reporter,

    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::ResolvedBy",
        to = "super::users::Column::Id",
        on_delete = "SetNull"
    )]
    ResolvedByUser,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

#[derive(Debug, Clone)]
pub struct ReporterLink;

impl Linked for ReporterLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Reporter.def()]
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedByLink;

impl Linked for ResolvedByLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::ResolvedByUser.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
