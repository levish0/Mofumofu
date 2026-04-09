use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::{Column as CommentsColumn, Entity as CommentsEntity};
use super::common::NotificationType;
use super::posts::{Column as PostsColumn, Entity as PostsEntity};
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_notifications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(nullable)]
    pub actor_id: Option<Uuid>,
    #[sea_orm(not_null)]
    pub notification_type: NotificationType,
    #[sea_orm(column_type = "Text", not_null)]
    pub action: String,
    #[sea_orm(nullable)]
    pub post_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub comment_id: Option<Uuid>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub additional_data: Option<Json>,
    #[sea_orm(column_type = "Boolean", not_null)]
    pub is_read: bool,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub read_at: Option<DateTimeUtc>,
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
        from = "Column::ActorId",
        to = "UsersColumn::Id",
        on_delete = "SetNull"
    )]
    Actor,
    #[sea_orm(
        belongs_to = "PostsEntity",
        from = "Column::PostId",
        to = "PostsColumn::Id",
        on_delete = "Cascade"
    )]
    Post,
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

impl Related<PostsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

pub struct ActorLink;

impl Linked for ActorLink {
    type FromEntity = Entity;
    type ToEntity = UsersEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Actor.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
