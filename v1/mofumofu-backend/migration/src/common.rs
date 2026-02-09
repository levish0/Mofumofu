use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum OAuthProvider {
    #[sea_orm(iden = "oauth_provider")]
    Table,
    #[sea_orm(iden = "google")]
    Google,
    #[sea_orm(iden = "github")]
    Github,
}

#[derive(DeriveIden, EnumIter)]
pub enum ActionType {
    #[sea_orm(iden = "action_type")]
    Table,
    #[sea_orm(iden = "user_created")]
    UserCreated,
    #[sea_orm(iden = "user_updated")]
    UserUpdated,
    #[sea_orm(iden = "user_signed_in")]
    UserSignedIn,
    #[sea_orm(iden = "user_signed_out")]
    UserSignedOut,
    #[sea_orm(iden = "post_created")]
    PostCreated,
    #[sea_orm(iden = "post_updated")]
    PostUpdated,
    #[sea_orm(iden = "post_deleted")]
    PostDeleted,
    #[sea_orm(iden = "post_viewed")]
    PostViewed,
    #[sea_orm(iden = "hashtag_created")]
    HashtagCreated,
    #[sea_orm(iden = "hashtag_used")]
    HashtagUsed,
    #[sea_orm(iden = "follow_created")]
    FollowCreated,
    #[sea_orm(iden = "follow_deleted")]
    FollowDeleted,
    #[sea_orm(iden = "like_created")]
    LikeCreated,
    #[sea_orm(iden = "like_deleted")]
    LikeDeleted,
    #[sea_orm(iden = "comment_created")]
    CommentCreated,
    #[sea_orm(iden = "comment_updated")]
    CommentUpdated,
    #[sea_orm(iden = "comment_deleted")]
    CommentDeleted,
}

#[derive(DeriveIden, EnumIter)]
pub enum TargetType {
    #[sea_orm(iden = "target_type")]
    Table,
    #[sea_orm(iden = "user")]
    User,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "hashtag")]
    Hashtag,
    #[sea_orm(iden = "comment")]
    Comment,
    #[sea_orm(iden = "follow")]
    Follow,
}

#[derive(DeriveIden, EnumIter)]
pub enum UserRole {
    #[sea_orm(iden = "user_role")]
    Table,
    #[sea_orm(iden = "member")]
    Member,
    #[sea_orm(iden = "moderator")]
    Moderator,
    #[sea_orm(iden = "admin")]
    Admin,
}

#[derive(DeriveIden, EnumIter)]
pub enum LikeTargetType {
    #[sea_orm(iden = "like_target_type")]
    Table,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "comment")]
    Comment,
}

#[derive(DeriveIden, EnumIter)]
pub enum ReportTargetType {
    #[sea_orm(iden = "report_target_type")]
    Table,
    #[sea_orm(iden = "user")]
    User,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "comment")]
    Comment,
}

#[derive(DeriveIden, EnumIter)]
pub enum ReportStatus {
    #[sea_orm(iden = "report_status")]
    Table,
    #[sea_orm(iden = "pending")]
    Pending,
    #[sea_orm(iden = "reviewing")]
    Reviewing,
    #[sea_orm(iden = "resolved")]
    Resolved,
    #[sea_orm(iden = "dismissed")]
    Dismissed,
}
