use sea_orm::prelude::*;
use uuid::Uuid;

use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "follows")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub follower_id: Uuid,
    #[sea_orm(not_null)]
    pub followee_id: Uuid,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::FollowerId",
        to = "UsersColumn::Id",
        on_delete = "Cascade"
    )]
    Follower,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::FolloweeId",
        to = "UsersColumn::Id",
        on_delete = "Cascade"
    )]
    Followee,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Follower.def()
    }
}

pub struct FolloweeLink;

impl Linked for FolloweeLink {
    type FromEntity = Entity;
    type ToEntity = UsersEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Followee.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
