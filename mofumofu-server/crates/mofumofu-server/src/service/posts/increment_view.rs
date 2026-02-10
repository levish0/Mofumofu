use crate::repository::posts::repository_increment_post_view_count;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_increment_post_view(
    conn: &DatabaseConnection,
    post_id: Uuid,
) -> ServiceResult<()> {
    repository_increment_post_view_count(conn, post_id).await?;
    Ok(())
}
