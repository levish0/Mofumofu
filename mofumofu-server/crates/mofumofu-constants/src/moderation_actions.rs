use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

/// Moderation Action enum (moderation_logs.action 필드에 저장됨)
/// 포맷: "{resource}:{operation}"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ModerationAction {
    // ==================== User Actions ====================
    /// 유저 밴
    #[serde(rename = "user:ban")]
    UserBan,
    /// 유저 밴 해제
    #[serde(rename = "user:unban")]
    UserUnban,
    /// 유저 역할 부여
    #[serde(rename = "user:grant_role")]
    UserGrantRole,
    /// 유저 역할 제거
    #[serde(rename = "user:revoke_role")]
    UserRevokeRole,

    // ==================== Report Actions ====================
    /// 신고 해결
    #[serde(rename = "report:resolve")]
    ReportResolve,
    /// 신고 기각
    #[serde(rename = "report:dismiss")]
    ReportDismiss,

    // ==================== Post Actions ====================
    /// 포스트 삭제 (모더레이션)
    #[serde(rename = "post:delete")]
    PostDelete,

    // ==================== Comment Actions ====================
    /// 댓글 삭제 (모더레이션)
    #[serde(rename = "comment:delete")]
    CommentDelete,
}

impl ModerationAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            // User
            ModerationAction::UserBan => "user:ban",
            ModerationAction::UserUnban => "user:unban",
            ModerationAction::UserGrantRole => "user:grant_role",
            ModerationAction::UserRevokeRole => "user:revoke_role",
            // Report
            ModerationAction::ReportResolve => "report:resolve",
            ModerationAction::ReportDismiss => "report:dismiss",
            // Post
            ModerationAction::PostDelete => "post:delete",
            // Comment
            ModerationAction::CommentDelete => "comment:delete",
        }
    }
}

impl fmt::Display for ModerationAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ModerationAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // User
            "user:ban" => Ok(ModerationAction::UserBan),
            "user:unban" => Ok(ModerationAction::UserUnban),
            "user:grant_role" => Ok(ModerationAction::UserGrantRole),
            "user:revoke_role" => Ok(ModerationAction::UserRevokeRole),
            // Report
            "report:resolve" => Ok(ModerationAction::ReportResolve),
            "report:dismiss" => Ok(ModerationAction::ReportDismiss),
            // Post
            "post:delete" => Ok(ModerationAction::PostDelete),
            // Comment
            "comment:delete" => Ok(ModerationAction::CommentDelete),
            _ => Err(format!("Unknown moderation action: {}", s)),
        }
    }
}

pub fn moderation_action_to_string(action: ModerationAction) -> String {
    action.as_str().to_string()
}

pub fn string_to_moderation_action(s: &str) -> Option<ModerationAction> {
    s.parse().ok()
}
