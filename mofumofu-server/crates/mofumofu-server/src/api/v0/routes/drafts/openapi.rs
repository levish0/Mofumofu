use mofumofu_dto::drafts::{
    DraftIdPath, DraftListResponse, DraftResponse, PublishDraftRequest, UpdateDraftRequest,
};
use mofumofu_dto::posts::PostResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_draft::create_draft,
        super::get_draft::get_draft,
        super::get_drafts::get_drafts,
        super::update_draft::update_draft,
        super::delete_draft::delete_draft,
        super::publish_draft::publish_draft,
    ),
    components(
        schemas(
            DraftResponse,
            DraftListResponse,
            UpdateDraftRequest,
            PublishDraftRequest,
            DraftIdPath,
            PostResponse,
        )
    ),
    tags(
        (name = "Drafts", description = "Draft endpoints")
    )
)]
pub struct DraftsApiDoc;
