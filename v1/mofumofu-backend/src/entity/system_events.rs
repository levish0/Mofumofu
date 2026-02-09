use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::{ActionType, TargetType};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "system_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub user_id: Option<Uuid>,

    #[sea_orm(not_null)]
    pub action_type: ActionType,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub target_id: Option<Uuid>,

    #[sea_orm(nullable)]
    pub target_type: Option<TargetType>,

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub metadata: Option<serde_json::Value>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
