use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::Entity as CommentsEntity;
use super::common::PostStatus;
use super::post_hashtags::Entity as PostHashtagsEntity;
use super::post_likes::Entity as PostLikesEntity;
use super::reports::Entity as ReportsEntity;
use super::user_notifications::Entity as UserNotificationsEntity;
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(not_null)]
    pub status: PostStatus,
    #[sea_orm(column_type = "Text", nullable)]
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub slug: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub thumbnail_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub render: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub toc: Option<Json>,
    #[sea_orm(not_null)]
    pub like_count: i32,
    #[sea_orm(not_null)]
    pub comment_count: i32,
    #[sea_orm(not_null)]
    pub view_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub published_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTimeUtc,
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
    #[sea_orm(has_many = "CommentsEntity")]
    Comments,
    #[sea_orm(has_many = "PostHashtagsEntity")]
    PostHashtags,
    #[sea_orm(has_many = "PostLikesEntity")]
    PostLikes,
    #[sea_orm(has_many = "ReportsEntity")]
    Reports,
    #[sea_orm(has_many = "UserNotificationsEntity")]
    UserNotifications,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<PostHashtagsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::PostHashtags.def()
    }
}

impl Related<PostLikesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::PostLikes.def()
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
