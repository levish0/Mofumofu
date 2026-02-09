use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// OAuth 제공자: 소셜 로그인 제공 업체
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "oauth_provider")]
pub enum OAuthProvider {
    /// Google OAuth
    #[sea_orm(string_value = "google")]
    Google,
    /// GitHub OAuth
    #[sea_orm(string_value = "github")]
    Github,
    /// Discord OAuth
    #[sea_orm(string_value = "discord")]
    Discord,
    /// X (Twitter) OAuth
    #[sea_orm(string_value = "x")]
    X,
    /// Microsoft OAuth
    #[sea_orm(string_value = "microsoft")]
    Microsoft,
}

/// 좋아요 대상 타입
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "like_target_type")]
pub enum LikeTargetType {
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
}

/// 사용자 역할
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "moderator")]
    Moderator,
    #[sea_orm(string_value = "admin")]
    Admin,
}

/// 신고 대상 타입
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "report_target_type"
)]
pub enum ReportTargetType {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
}

/// 신고 처리 상태
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
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

/// 모더레이션 리소스 타입
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "moderation_resource_type"
)]
pub enum ModerationResourceType {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
    #[sea_orm(string_value = "system")]
    System,
}

/// Action Log 리소스 타입: 사용자 활동 로그의 대상 리소스 종류
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "action_resource_type"
)]
pub enum ActionResourceType {
    /// 사용자
    #[sea_orm(string_value = "user")]
    User,
    /// 포스트
    #[sea_orm(string_value = "post")]
    Post,
}
