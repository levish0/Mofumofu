use crate::entity::common::UserRole;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(string_len = 20, not_null)]
    pub name: String,
    #[sea_orm(string_len = 20, not_null, unique)]
    pub handle: String, // Unique
    #[sea_orm(string_len = 200, nullable)]
    pub bio: Option<String>,
    #[sea_orm(string_len = 30, nullable)]
    pub location: Option<String>,
    #[sea_orm(string_len = 50, nullable)]
    pub website: Option<String>,
    #[sea_orm(string_len = 254, not_null, unique)]
    pub email: String, // Unique
    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub is_verified: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub banner_image: Option<String>,
    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub follower_count: i32,
    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub following_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    pub role: UserRole,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 작성한 포스트들
    #[sea_orm(
        has_many = "super::posts::Entity",
        from = "Column::Id",
        to = "super::posts::Column::UserId"
    )]
    Posts,

    // 작성한 댓글들
    #[sea_orm(
        has_many = "super::comments::Entity",
        from = "Column::Id",
        to = "super::comments::Column::UserId"
    )]
    Comments,

    // 리프레시 토큰들
    #[sea_orm(
        has_many = "super::user_refresh_tokens::Entity",
        from = "Column::Id",
        to = "super::user_refresh_tokens::Column::UserId"
    )]
    RefreshTokens,

    // 이 유저를 팔로우하는 사람들 (followers)
    #[sea_orm(
        has_many = "super::follows::Entity",
        from = "Column::Id",
        to = "super::follows::Column::FolloweeId"
    )]
    Followers,

    // 이 유저가 팔로우하는 사람들 (following)
    #[sea_orm(
        has_many = "super::follows::Entity",
        from = "Column::Id",
        to = "super::follows::Column::FollowerId"
    )]
    Following,

    #[sea_orm(
        has_many = "super::user_oauth_connections::Entity",
        from = "Column::Id",
        to = "super::user_oauth_connections::Column::UserId"
    )]
    OAuthConnections,

    // 작성한 임시저장들
    #[sea_orm(
        has_many = "super::drafts::Entity",
        from = "Column::Id",
        to = "super::drafts::Column::UserId"
    )]
    Drafts,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::user_refresh_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl Related<super::follows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Followers.def()
    }
}

impl Related<super::user_oauth_connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OAuthConnections.def()
    }
}

impl Related<super::drafts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Drafts.def()
    }
}

// Linked 구현: 이 유저의 팔로워들 가져오기
#[derive(Debug, Clone)]
pub struct GetFollowersLink;

impl Linked for GetFollowersLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Followers.def(),                // users -> follows (followers)
            super::follows::Relation::Follower.def(), // follows -> users (follower info)
        ]
    }
}

// Linked 구현: 이 유저가 팔로우하는 사람들 가져오기
#[derive(Debug, Clone)]
pub struct GetFollowingLink;

impl Linked for GetFollowingLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Following.def(),                // users -> follows (following)
            super::follows::Relation::Followee.def(), // follows -> users (followee info)
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
