use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "oauth_provider")]
pub enum OAuthProvider {
    #[sea_orm(string_value = "google")]
    Google,

    #[sea_orm(string_value = "github")]
    Github,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "action_type")]
pub enum ActionType {
    #[sea_orm(string_value = "user_created")]
    UserCreated,
    #[sea_orm(string_value = "user_updated")]
    UserUpdated,
    #[sea_orm(string_value = "user_signed_in")]
    UserSignedIn,
    #[sea_orm(string_value = "user_signed_out")]
    UserSignedOut,
    #[sea_orm(string_value = "post_created")]
    PostCreated,
    #[sea_orm(string_value = "post_updated")]
    PostUpdated,
    #[sea_orm(string_value = "post_deleted")]
    PostDeleted,
    #[sea_orm(string_value = "post_viewed")]
    PostViewed,
    #[sea_orm(string_value = "hashtag_created")]
    HashtagCreated,
    #[sea_orm(string_value = "hashtag_used")]
    HashtagUsed,
    #[sea_orm(string_value = "follow_created")]
    FollowCreated,
    #[sea_orm(string_value = "follow_deleted")]
    FollowDeleted,
    #[sea_orm(string_value = "comment_created")]
    CommentCreated,
    #[sea_orm(string_value = "comment_updated")]
    CommentUpdated,
    #[sea_orm(string_value = "comment_deleted")]
    CommentDeleted,
    #[sea_orm(string_value = "like_created")]
    LikeCreated,
    #[sea_orm(string_value = "like_deleted")]
    LikeDeleted,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "target_type")]
pub enum TargetType {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "hashtag")]
    Hashtag,
    #[sea_orm(string_value = "comment")]
    Comment,
    #[sea_orm(string_value = "follow")]
    Follow,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "member")]
    Member,
    #[sea_orm(string_value = "moderator")]
    Moderator,
    #[sea_orm(string_value = "admin")]
    Admin,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "like_target_type")]
pub enum LikeTargetType {
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_target_type")]
pub enum ReportTargetType {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, ToSchema)]
pub enum ReportReason {
    Spam,
    InappropriateContent,
    Harassment,
    Copyright,
    FakeInformation,
    Violence,
    AdultContent,
    Other,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_status")]
pub enum ReportStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "reviewing")]
    Reviewing,
    #[sea_orm(string_value = "resolved")]
    Resolved,
    #[sea_orm(string_value = "dismissed")]
    Dismissed,
}
