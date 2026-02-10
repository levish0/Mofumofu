use crate::repository::drafts::repository_find_drafts_by_user_id;
use mofumofu_dto::drafts::{DraftListResponse, DraftResponse};
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_drafts(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> ServiceResult<DraftListResponse> {
    let drafts = repository_find_drafts_by_user_id(conn, user_id).await?;

    let data: Vec<DraftResponse> = drafts.into_iter().map(DraftResponse::from).collect();

    Ok(DraftListResponse { data })
}
