use sea_orm::prelude::*;
use uuid::Uuid;

use super::common::ModerationResourceType;
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "moderation_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", not_null)]
    pub action: String,
    #[sea_orm(nullable)]
    pub actor_id: Option<Uuid>,
    #[sea_orm(not_null)]
    pub resource_type: ModerationResourceType,
    #[sea_orm(nullable)]
    pub resource_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub reason: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub metadata: Option<Json>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::ActorId",
        to = "UsersColumn::Id",
        on_delete = "SetNull"
    )]
    Actor,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
