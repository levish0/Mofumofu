use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::{Column as CommentsColumn, Entity as CommentsEntity};
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comment_likes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(not_null)]
    pub comment_id: Uuid,
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
        belongs_to = "CommentsEntity",
        from = "Column::CommentId",
        to = "CommentsColumn::Id",
        on_delete = "Cascade"
    )]
    Comment,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
