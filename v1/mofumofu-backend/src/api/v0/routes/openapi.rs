use crate::dto::admin::response::{AdminStatusResponse, AdminTaskResponse};
use crate::dto::auth::request::forgot_password::ForgotPasswordRequest;
use crate::dto::auth::request::link_oauth::LinkOAuthRequest;
use crate::dto::auth::request::login::AuthLoginRequest;
use crate::dto::auth::request::resend_verification::ResendVerificationRequest;
use crate::dto::auth::request::reset_password::ResetPasswordRequest;
use crate::dto::auth::request::set_password::SetPasswordRequest;
use crate::dto::auth::request::unlink_oauth::UnlinkOAuthRequest;
use crate::dto::auth::request::verify_email::VerifyEmailRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::dto::auth::response::oauth_connections::OAuthConnectionsResponse;
use crate::dto::comment::request::create_comment::CreateCommentRequest;
use crate::dto::comment::request::delete_comment::DeleteCommentRequest;
use crate::dto::comment::request::get_comment_by_id::GetCommentByIdRequest;
use crate::dto::comment::request::get_comments::{GetCommentsRequest, GetRepliesRequest};
use crate::dto::comment::request::update_comment::UpdateCommentRequest;
use crate::dto::comment::response::comment_info::CommentInfo;
use crate::dto::comment::response::get_comments::{GetCommentsResponse, GetRepliesResponse};
use crate::dto::draft::request::create_draft::CreateDraftRequest;
use crate::dto::draft::request::delete_draft::DeleteDraftRequest;
use crate::dto::draft::request::get_draft::GetDraftRequest;
use crate::dto::draft::request::update_draft::UpdateDraftRequest;
use crate::dto::draft::response::create_draft::CreateDraftResponse;
use crate::dto::draft::response::draft_info::DraftInfo;
use crate::dto::draft::response::get_drafts::GetDraftsResponse;
use crate::dto::follow::request::check_follow_status::CheckFollowStatusRequest;
use crate::dto::follow::request::create::CreateFollowRequest;
use crate::dto::follow::request::delete::DeleteFollowRequest;
use crate::dto::follow::request::get_count::GetFollowCountRequest;
use crate::dto::follow::response::follow_count::FollowCountResponse;
use crate::dto::follow::response::follow_list::FollowListResponse;
use crate::dto::follow::response::follow_status::FollowStatusResponse;
use crate::dto::hashtag::request::trending_hashtags::TrendingHashtagsRequest;
use crate::dto::hashtag::response::trending_hashtags::TrendingHashtagsResponse;
use crate::dto::like::request::check_comment_like_status::CheckCommentLikeStatusRequest;
use crate::dto::like::request::check_like_status::CheckLikeStatusRequest;
use crate::dto::like::request::create_comment_like::CreateCommentLikeRequest;
use crate::dto::like::request::create_like::CreateLikeRequest;
use crate::dto::like::request::delete_comment_like::DeleteCommentLikeRequest;
use crate::dto::like::request::delete_like::DeleteLikeRequest;
use crate::dto::like::response::like_status::LikeStatusResponse;
use crate::dto::post::request::GetPostByHandleAndSlugRequest;
use crate::dto::post::request::GetPostByUuidRequest;
use crate::dto::post::request::create_post::CreatePostRequest;
use crate::dto::post::request::delete_post::DeletePostRequest;
use crate::dto::post::request::get_post_for_edit::GetPostForEditRequest;
use crate::dto::post::request::image_upload::ImageUploadForm;
use crate::dto::post::request::thumbnail_image::PostThumbnailForm;
use crate::dto::post::request::update_post::UpdatePostRequest;
use crate::dto::post::request::{
    GetPostsRequest, GetUserPostsRequest, PostSortOrder, SearchPostsRequest,
};
use crate::dto::post::response::create_post::CreatePostResponse;
use crate::dto::post::response::post_edit_info::PostEditInfoResponse;
use crate::dto::post::response::post_info::{PostAuthor, PostInfoResponse};
use crate::dto::post::response::{
    GetPostsResponse, ImageUploadResponse, PostListItem, UserPostsResponse,
};
use crate::dto::report::request::{CreateReportRequest, GetReportsRequest, ProcessReportRequest};
use crate::dto::report::response::{CreateReportResponse, GetReportsResponse, ReportInfo};
use crate::dto::user::request::avatar_image::ProfileAvatarForm;
use crate::dto::user::request::banner_image::ProfileBannerForm;
use crate::dto::user::request::create::CreateUserRequest;
use crate::dto::user::request::get_profile::GetUserProfileRequest;
use crate::dto::user::request::update_profile::UpdateProfileRequest;
use crate::dto::user::response::handle_check::HandleCheckResponse;
use crate::dto::user::response::info::UserInfoResponse;
use crate::entity::common::{OAuthProvider, ReportReason, ReportStatus, ReportTargetType};
use crate::service::error::errors::ErrorResponse;
use utoipa::openapi::security::{ApiKey, ApiKeyValue};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::v0::routes::auth::forgot_password::forgot_password,
        crate::api::v0::routes::auth::get_oauth_connections::get_oauth_connections,
        crate::api::v0::routes::auth::github::github_sign_in,
        crate::api::v0::routes::auth::google::google_sign_in,
        crate::api::v0::routes::auth::link_oauth::link_oauth,
        crate::api::v0::routes::auth::resend_verification::resend_verification,
        crate::api::v0::routes::auth::reset_password::reset_password,
        crate::api::v0::routes::auth::set_password::set_password,
        crate::api::v0::routes::auth::sign_in::sign_in,
        crate::api::v0::routes::auth::sign_out::sign_out,
        crate::api::v0::routes::auth::sign_up::sign_up,
        crate::api::v0::routes::auth::unlink_oauth::unlink_oauth,
        crate::api::v0::routes::auth::verify_email::verify_email,
        crate::api::v0::routes::auth::refresh::refresh,
        crate::api::v0::routes::user::get_my_profile::get_my_profile,
        crate::api::v0::routes::user::check_handle::check_handle_availability,
        crate::api::v0::routes::user::get_profile::get_profile,
        crate::api::v0::routes::user::update_profile::update_profile,
        crate::api::v0::routes::user::upload_avatar::upload_avatar,
        crate::api::v0::routes::user::upload_banner::upload_banner,
        crate::api::v0::routes::post::create_post::create_post,
        crate::api::v0::routes::post::delete_post::delete_post,
        crate::api::v0::routes::post::get_post::get_post,
        crate::api::v0::routes::post::get_post_by_handle_and_slug::get_post_by_handle_and_slug,
        crate::api::v0::routes::post::get_post_for_edit::get_post_for_edit,
        crate::api::v0::routes::post::update_post::update_post,
        crate::api::v0::routes::post::get_posts::get_posts,
        crate::api::v0::routes::post::get_user_posts::get_user_posts,
        crate::api::v0::routes::post::increment_view::increment_view,
        crate::api::v0::routes::post::search_posts::search_posts,
        crate::api::v0::routes::post::upload_image::upload_image,
        crate::api::v0::routes::post::upload_thumbnail::upload_thumbnail,
        crate::api::v0::routes::follow::check_follow_status::api_check_follow_status,
        crate::api::v0::routes::follow::create_follow::api_create_follow,
        crate::api::v0::routes::follow::delete_follow::api_delete_follow,
        crate::api::v0::routes::follow::get_follower_count::api_get_follower_count,
        crate::api::v0::routes::follow::get_followers_list::get_followers,
        crate::api::v0::routes::follow::get_following_count::api_get_following_count,
        crate::api::v0::routes::follow::get_following_list::get_following,
        crate::api::v0::routes::hashtag::trending_hashtags::trending_hashtags,
        crate::api::v0::routes::like::check_like_status::check_like_status,
        crate::api::v0::routes::like::create_like::create_like,
        crate::api::v0::routes::like::delete_like::delete_like,
        crate::api::v0::routes::like::check_comment_like_status::check_comment_like_status,
        crate::api::v0::routes::like::create_comment_like::create_comment_like,
        crate::api::v0::routes::like::delete_comment_like::delete_comment_like,
        crate::api::v0::routes::comment::create_comment::create_comment,
        crate::api::v0::routes::comment::delete_comment::delete_comment,
        crate::api::v0::routes::comment::get_comment_by_id::get_comment_by_id,
        crate::api::v0::routes::comment::get_comments::get_comments,
        crate::api::v0::routes::comment::get_replies::get_replies,
        crate::api::v0::routes::comment::update_comment::update_comment,
        crate::api::v0::routes::draft::create_draft::create_draft,
        crate::api::v0::routes::draft::delete_draft::delete_draft,
        crate::api::v0::routes::draft::get_draft::get_draft,
        crate::api::v0::routes::draft::get_drafts::get_drafts,
        crate::api::v0::routes::draft::update_draft::update_draft,
        crate::api::v0::routes::report::create_report::create_report,
        crate::api::v0::routes::report::get_reports::get_reports,
        crate::api::v0::routes::report::process_report::process_report,
        // Admin endpoints
        crate::api::v0::routes::admin::check_admin_status::check_admin_status,
        crate::api::v0::routes::admin::reindex_all_posts::reindex_all_posts,
        crate::api::v0::routes::admin::meilisearch_health::meilisearch_health,
        crate::api::v0::routes::admin::search_stats::search_stats,
        crate::api::v0::routes::admin::sync_likes::sync_likes,
        crate::api::v0::routes::admin::sync_follows::sync_follows,
        crate::api::v0::routes::admin::sync_all_counts::sync_all_counts,
        crate::api::v0::routes::admin::cleanup_expired_tokens::cleanup_expired_tokens,
        crate::api::v0::routes::admin::cleanup_old_events::cleanup_old_events
    ),
    components(
        schemas(
            AuthLoginRequest,
            AuthJWTResponse,
            ForgotPasswordRequest,
            ResendVerificationRequest,
            ResetPasswordRequest,
            UnlinkOAuthRequest,
            OAuthConnectionsResponse,
            SetPasswordRequest,
            LinkOAuthRequest,
            VerifyEmailRequest,
            OAuthProvider,
            CreateUserRequest,
            CreatePostRequest,
            DeletePostRequest,
            GetPostByUuidRequest,
            GetPostByHandleAndSlugRequest,
            GetPostForEditRequest,
            UpdatePostRequest,
            GetPostsRequest,
            GetUserPostsRequest,
            SearchPostsRequest,
            CreatePostResponse,
            PostSortOrder,
            PostEditInfoResponse,
            PostInfoResponse,
            PostAuthor,
            PostListItem,
            GetPostsResponse,
            ImageUploadResponse,
            UserPostsResponse,
            CheckFollowStatusRequest,
            CreateFollowRequest,
            DeleteFollowRequest,
            GetFollowCountRequest,
            FollowCountResponse,
            FollowListResponse,
            FollowStatusResponse,
            GetUserProfileRequest,
            UpdateProfileRequest,
            HandleCheckResponse,
            UserInfoResponse,
            ErrorResponse,
            ImageUploadForm,
            ProfileAvatarForm,
            ProfileBannerForm,
            PostThumbnailForm,
            TrendingHashtagsRequest,
            TrendingHashtagsResponse,
            CheckLikeStatusRequest,
            CreateLikeRequest,
            DeleteLikeRequest,
            CheckCommentLikeStatusRequest,
            CreateCommentLikeRequest,
            DeleteCommentLikeRequest,
            LikeStatusResponse,
            CreateCommentRequest,
            DeleteCommentRequest,
            GetCommentByIdRequest,
            GetCommentsRequest,
            GetRepliesRequest,
            UpdateCommentRequest,
            CommentInfo,
            GetCommentsResponse,
            GetRepliesResponse,
            CreateDraftRequest,
            DeleteDraftRequest,
            GetDraftRequest,
            UpdateDraftRequest,
            CreateDraftResponse,
            DraftInfo,
            GetDraftsResponse,
            CreateReportRequest,
            GetReportsRequest,
            ProcessReportRequest,
            CreateReportResponse,
            GetReportsResponse,
            ReportInfo,
            ReportReason,
            ReportStatus,
            ReportTargetType,
            // Admin schemas
            AdminStatusResponse,
            AdminTaskResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User", description = "User endpoints"),
        (name = "Post", description = "Post endpoints"),
        (name = "Draft", description = "Draft management endpoints"),
        (name = "Comment", description = "Comment endpoints"),
        (name = "Follow", description = "Follow endpoints"),
        (name = "Hashtag", description = "Hashtag endpoints"),
        (name = "Like", description = "Like endpoints"),
        (name = "Report", description = "Report endpoints"),
        (name = "Admin", description = "Admin management endpoints")
    ),
    modifiers(&SecurityAddon) // 보안 스키마 등록
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );

            components.add_security_scheme(
                "refresh_token_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("refresh_token"))),
            );

            components.add_security_scheme(
                "anonymous_id_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("anonymous_user_id"))),
            )
        }
    }
}
