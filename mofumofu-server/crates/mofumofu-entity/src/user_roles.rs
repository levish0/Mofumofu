use sea_orm::prelude::*;
use uuid::Uuid;

use super::common::UserRole;
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(not_null)]
    pub role: UserRole,
    #[sea_orm(nullable)]
    pub granted_by: Option<Uuid>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::UserId",
        to = "UsersColumn::Id",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::GrantedBy",
        to = "UsersColumn::Id",
        on_delete = "SetNull"
    )]
    GrantedByUser,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

pub struct GrantedByLink;

impl Linked for GrantedByLink {
    type FromEntity = Entity;
    type ToEntity = UsersEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::GrantedByUser.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
