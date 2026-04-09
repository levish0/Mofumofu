use sea_orm::prelude::*;
use uuid::Uuid;

use super::comment_likes::Entity as CommentLikesEntity;
use super::posts::{Column as PostsColumn, Entity as PostsEntity};
use super::reports::Entity as ReportsEntity;
use super::user_notifications::Entity as UserNotificationsEntity;
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub post_id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(nullable)]
    pub parent_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", not_null)]
    pub content: String,
    #[sea_orm(not_null)]
    pub like_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub deleted_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "PostsEntity",
        from = "Column::PostId",
        to = "PostsColumn::Id",
        on_delete = "Cascade"
    )]
    Post,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::UserId",
        to = "UsersColumn::Id",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_delete = "NoAction"
    )]
    ParentComment,
    #[sea_orm(has_many = "Entity", from = "Column::Id", to = "Column::ParentId")]
    ChildComments,
    #[sea_orm(has_many = "CommentLikesEntity")]
    CommentLikes,
    #[sea_orm(has_many = "ReportsEntity")]
    Reports,
    #[sea_orm(has_many = "UserNotificationsEntity")]
    UserNotifications,
}

impl Related<PostsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParentComment.def()
    }
}

impl Related<CommentLikesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::CommentLikes.def()
    }
}

impl Related<ReportsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Reports.def()
    }
}

impl Related<UserNotificationsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserNotifications.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
