use super::action_logs::routes::action_logs_routes as ActionLogsRoutes;
use super::auth::routes::auth_routes as AuthRoutes;
use super::comments::routes::comments_routes as CommentsRoutes;
use super::drafts::routes::drafts_routes as DraftsRoutes;
use super::follows::routes::follows_routes as FollowsRoutes;
use super::hashtags::routes::hashtags_routes as HashtagsRoutes;
use super::likes::routes::likes_routes as LikesRoutes;
use super::moderation_logs::routes::moderation_logs_routes as ModerationLogsRoutes;
use super::posts::routes::posts_routes as PostsRoutes;
use super::reports::routes::reports_routes as ReportsRoutes;
use super::search::routes::search_routes as SearchRoutes;
use super::stream::routes::stream_routes as StreamRoutes;
use super::user::routes::user_routes as UserRoutes;
use super::user_bans::routes::user_bans_routes as UserBansRoutes;
use super::user_roles::routes::user_roles_routes as UserRolesRoutes;
use crate::state::AppState;
use axum::Router;

/// v0 API 라우터
pub fn v0_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(UserRoutes())
        .merge(AuthRoutes(state.clone()))
        .merge(SearchRoutes())
        .merge(ActionLogsRoutes())
        .merge(StreamRoutes())
        .merge(PostsRoutes())
        .merge(DraftsRoutes())
        .merge(CommentsRoutes())
        .merge(LikesRoutes())
        .merge(FollowsRoutes())
        .merge(HashtagsRoutes())
        .merge(ReportsRoutes())
        .merge(UserRolesRoutes())
        .merge(UserBansRoutes())
        .merge(ModerationLogsRoutes())
}
