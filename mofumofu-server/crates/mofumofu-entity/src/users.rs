use sea_orm::prelude::*;
use uuid::Uuid;

use super::action_logs::Entity as ActionLogsEntity;
use super::comment_likes::Entity as CommentLikesEntity;
use super::comments::Entity as CommentsEntity;
use super::moderation_logs::Entity as ModerationLogsEntity;
use super::notification_action_preferences::Entity as NotificationActionPreferencesEntity;
use super::notification_preferences::Entity as NotificationPreferencesEntity;
use super::post_likes::Entity as PostLikesEntity;
use super::posts::Entity as PostsEntity;
use super::user_bans::Entity as UserBansEntity;
use super::user_oauth_connections::Entity as UserOAuthConnectionsEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", not_null, unique)]
    pub handle: String,
    #[sea_orm(column_type = "Text", not_null)]
    pub display_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub bio: Option<String>,
    #[sea_orm(column_type = "Text", not_null, unique)]
    pub email: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub banner_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub totp_secret: Option<String>,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub totp_enabled_at: Option<DateTimeUtc>,
    #[sea_orm(nullable)]
    pub totp_backup_codes: Option<Vec<String>>,
    #[sea_orm(not_null)]
    pub follower_count: i32,
    #[sea_orm(not_null)]
    pub following_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "UserOAuthConnectionsEntity")]
    OAuthConnections,
    #[sea_orm(has_many = "PostsEntity")]
    Posts,
    #[sea_orm(has_many = "CommentsEntity")]
    Comments,
    #[sea_orm(has_many = "PostLikesEntity")]
    PostLikes,
    #[sea_orm(has_many = "CommentLikesEntity")]
    CommentLikes,
    #[sea_orm(has_many = "UserBansEntity")]
    UserBans,
    #[sea_orm(has_many = "ActionLogsEntity")]
    ActionLogs,
    #[sea_orm(has_many = "ModerationLogsEntity")]
    ModerationLogs,
    #[sea_orm(has_many = "NotificationPreferencesEntity")]
    NotificationPreferences,
    #[sea_orm(has_many = "NotificationActionPreferencesEntity")]
    NotificationActionPreferences,
}

impl Related<UserOAuthConnectionsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::OAuthConnections.def()
    }
}

impl Related<PostsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<PostLikesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::PostLikes.def()
    }
}

impl Related<CommentLikesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::CommentLikes.def()
    }
}

impl Related<UserBansEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBans.def()
    }
}

impl Related<ActionLogsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::ActionLogs.def()
    }
}

impl Related<ModerationLogsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::ModerationLogs.def()
    }
}

impl Related<NotificationPreferencesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::NotificationPreferences.def()
    }
}

impl Related<NotificationActionPreferencesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::NotificationActionPreferences.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
