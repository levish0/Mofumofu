use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "follows")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub follower_id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub followee_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::FollowerId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Follower,

    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::FolloweeId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Followee,
}

// Follows -> Users 관계 구현 (follower 방향)
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Follower.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
