use super::publish_job;
use crate::state::WorkerClient;
use mofumofu_errors::errors::Errors;
use mofumofu_worker::jobs::user_index::{IndexUserJob, UserIndexAction};
use mofumofu_worker::nats::streams::INDEX_USER_SUBJECT;
use tracing::info;
use uuid::Uuid;

/// Push a user indexing job to the worker queue
pub async fn index_user(worker: &WorkerClient, user_id: Uuid) -> Result<(), Errors> {
    info!("Queuing user index job for {}", user_id);

    let job = IndexUserJob {
        user_id,
        action: UserIndexAction::Index,
    };

    publish_job(worker, INDEX_USER_SUBJECT, &job).await?;

    info!("User index job queued for {}", user_id);
    Ok(())
}

/// Push a user deletion job to the worker queue
pub async fn delete_user_from_index(worker: &WorkerClient, user_id: Uuid) -> Result<(), Errors> {
    info!("Queuing user delete job for {}", user_id);

    let job = IndexUserJob {
        user_id,
        action: UserIndexAction::Delete,
    };

    publish_job(worker, INDEX_USER_SUBJECT, &job).await?;

    info!("User delete job queued for {}", user_id);
    Ok(())
}
