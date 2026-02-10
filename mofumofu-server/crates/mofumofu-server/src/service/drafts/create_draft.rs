use crate::repository::drafts::repository_create_draft;
use mofumofu_dto::drafts::DraftResponse;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_create_draft(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> ServiceResult<DraftResponse> {
    let draft = repository_create_draft(conn, user_id).await?;

    Ok(DraftResponse::from(draft))
}
