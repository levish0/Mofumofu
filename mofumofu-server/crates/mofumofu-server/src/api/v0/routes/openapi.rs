use super::action_logs::openapi::ActionLogsOpenApi;
use super::auth::openapi::AuthApiDoc;
use super::comments::openapi::CommentsApiDoc;
use super::drafts::openapi::DraftsApiDoc;
use super::follows::openapi::FollowsApiDoc;
use super::hashtags::openapi::HashtagsApiDoc;
use super::likes::openapi::LikesApiDoc;
use super::moderation_logs::openapi::ModerationLogsApiDoc;
use super::posts::openapi::PostsApiDoc;
use super::reports::openapi::ReportsApiDoc;
use super::search::openapi::SearchApiDoc;
use super::stream::openapi::StreamOpenApi;
use super::user::openapi::UserApiDoc;
use super::user_bans::openapi::UserBansApiDoc;
use super::user_roles::openapi::UserRolesApiDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi()]
pub struct V0ApiDoc;

impl V0ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(AuthApiDoc::openapi());
        openapi.merge(UserApiDoc::openapi());
        openapi.merge(SearchApiDoc::openapi());
        openapi.merge(ActionLogsOpenApi::openapi());
        openapi.merge(StreamOpenApi::openapi());
        openapi.merge(PostsApiDoc::openapi());
        openapi.merge(DraftsApiDoc::openapi());
        openapi.merge(CommentsApiDoc::openapi());
        openapi.merge(LikesApiDoc::openapi());
        openapi.merge(FollowsApiDoc::openapi());
        openapi.merge(HashtagsApiDoc::openapi());
        openapi.merge(ReportsApiDoc::openapi());
        openapi.merge(UserRolesApiDoc::openapi());
        openapi.merge(UserBansApiDoc::openapi());
        openapi.merge(ModerationLogsApiDoc::openapi());
        openapi
    }
}
