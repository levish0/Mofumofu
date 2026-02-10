use crate::repository::comments::repository_get_comment_by_id;
use crate::repository::posts::repository_get_post_by_id;
use crate::repository::reports::repository_create_report;
use crate::repository::user::repository_get_user_by_id;
use mofumofu_dto::reports::{CreateReportRequest, ReportResponse};
use mofumofu_entity::common::ReportTargetType;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_create_report(
    conn: &DatabaseConnection,
    reporter_id: Uuid,
    payload: CreateReportRequest,
) -> ServiceResult<ReportResponse> {
    // Validate target exists
    match payload.target_type {
        ReportTargetType::User => {
            repository_get_user_by_id(conn, payload.target_id).await?;
        }
        ReportTargetType::Post => {
            repository_get_post_by_id(conn, payload.target_id)
                .await
                .map_err(|_| Errors::LikeTargetNotFound)?;
        }
        ReportTargetType::Comment => {
            repository_get_comment_by_id(conn, payload.target_id)
                .await
                .map_err(|_| Errors::LikeTargetNotFound)?;
        }
    }

    let report = repository_create_report(
        conn,
        reporter_id,
        payload.target_type,
        payload.target_id,
        payload.reason,
        payload.description,
    )
    .await?;

    Ok(ReportResponse::from(report))
}
