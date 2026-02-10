use mofumofu_dto::moderation_logs::{
    GetModerationLogsRequest, ModerationLogListResponse, ModerationLogResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_moderation_logs::get_moderation_logs,
    ),
    components(
        schemas(
            ModerationLogResponse,
            ModerationLogListResponse,
            GetModerationLogsRequest,
        )
    ),
    tags(
        (name = "Moderation Logs", description = "Moderation log endpoints")
    )
)]
pub struct ModerationLogsApiDoc;
