use sea_orm::prelude::*;
use uuid::Uuid;

use super::users::Entity as UsersEntity;

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
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Follower,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::FolloweeId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Followee,
}

impl ActiveModelBehavior for ActiveModel {}
