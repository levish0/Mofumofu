use crate::entity::common::LikeTargetType;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "likes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub user_id: Uuid,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub post_id: Option<Uuid>,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub comment_id: Option<Uuid>,

    pub target_type: LikeTargetType,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    User,

    #[sea_orm(
        belongs_to = "super::posts::Entity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_delete = "Cascade"
    )]
    Post,

    #[sea_orm(
        belongs_to = "super::comments::Entity",
        from = "Column::CommentId",
        to = "super::comments::Column::Id",
        on_delete = "Cascade"
    )]
    Comment,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
