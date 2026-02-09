use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::Entity as CommentsEntity;
use super::drafts::Entity as DraftsEntity;
use super::follows::Entity as FollowsEntity;
use super::likes::Entity as LikesEntity;
use super::moderation_logs::Entity as ModerationLogsEntity;
use super::posts::Entity as PostsEntity;
use super::reports::Entity as ReportsEntity;
use super::user_bans::Entity as UserBansEntity;
use super::user_oauth_connections::Entity as UserOAuthConnectionsEntity;
use super::user_roles::Entity as UserRolesEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_name = "display_name", not_null)]
    pub display_name: String,
    #[sea_orm(string_len = 20, not_null, unique)]
    pub handle: String,
    #[sea_orm(string_len = 200, nullable)]
    pub bio: Option<String>,
    #[sea_orm(string_len = 254, not_null, unique)]
    pub email: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub verified_at: Option<DateTimeUtc>,
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "UserOAuthConnectionsEntity")]
    OAuthConnections,
    #[sea_orm(has_many = "PostsEntity")]
    Posts,
    #[sea_orm(has_many = "DraftsEntity")]
    Drafts,
    #[sea_orm(has_many = "CommentsEntity")]
    Comments,
    #[sea_orm(has_many = "UserRolesEntity")]
    UserRoles,
    #[sea_orm(has_many = "UserBansEntity")]
    UserBans,
    #[sea_orm(has_many = "FollowsEntity")]
    Follows,
    #[sea_orm(has_many = "LikesEntity")]
    Likes,
    #[sea_orm(has_many = "ReportsEntity")]
    Reports,
    #[sea_orm(has_many = "ModerationLogsEntity")]
    ModerationLogs,
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

impl Related<DraftsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Drafts.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<UserRolesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<UserBansEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBans.def()
    }
}

impl Related<FollowsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Follows.def()
    }
}

impl Related<LikesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Likes.def()
    }
}

impl Related<ReportsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Reports.def()
    }
}

impl Related<ModerationLogsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::ModerationLogs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
