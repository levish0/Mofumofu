use crate::repository::drafts::repository_get_draft_by_id;
use mofumofu_dto::drafts::DraftResponse;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_draft(
    conn: &DatabaseConnection,
    user_id: Uuid,
    draft_id: Uuid,
) -> ServiceResult<DraftResponse> {
    let draft = repository_get_draft_by_id(conn, draft_id).await?;

    if draft.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    Ok(DraftResponse::from(draft))
}
