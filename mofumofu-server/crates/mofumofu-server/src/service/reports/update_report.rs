use crate::repository::moderation_logs::repository_create_moderation_log;
use crate::repository::reports::{
    ReportUpdateParams, repository_get_report_by_id, repository_update_report,
};
use mofumofu_constants::ModerationAction;
use mofumofu_dto::reports::{ReportResponse, UpdateReportRequest};
use mofumofu_entity::common::{ModerationResourceType, ReportStatus};
use mofumofu_errors::errors::ServiceResult;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_update_report(
    conn: &DatabaseConnection,
    actor_id: Uuid,
    report_id: Uuid,
    payload: UpdateReportRequest,
) -> ServiceResult<ReportResponse> {
    let txn = conn.begin().await?;

    let _report = repository_get_report_by_id(&txn, report_id).await?;

    let is_resolving = matches!(
        payload.status,
        Some(ReportStatus::Resolved) | Some(ReportStatus::Dismissed)
    );

    let params = ReportUpdateParams {
        status: payload.status.clone(),
        resolved_by: if is_resolving {
            Some(Some(actor_id))
        } else {
            None
        },
        resolved_at: if is_resolving {
            Some(Some(chrono::Utc::now()))
        } else {
            None
        },
    };

    let updated = repository_update_report(&txn, report_id, params).await?;

    if let Some(ref status) = payload.status {
        let action = match status {
            ReportStatus::Resolved => ModerationAction::ReportResolve,
            ReportStatus::Dismissed => ModerationAction::ReportDismiss,
            _ => {
                txn.commit().await?;
                return Ok(ReportResponse::from(updated));
            }
        };
        let _ = repository_create_moderation_log(
            &txn,
            action,
            actor_id,
            ModerationResourceType::User,
            Some(report_id),
            payload.reason,
            None,
        )
        .await;
    }

    txn.commit().await?;

    Ok(ReportResponse::from(updated))
}
