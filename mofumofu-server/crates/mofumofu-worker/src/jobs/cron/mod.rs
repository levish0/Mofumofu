mod cleanup;

use crate::DbPool;
use crate::config::WorkerConfig;
use chrono_tz::Tz;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobSchedulerError};

/// Cleanup cron schedule: 4:00 AM every Saturday
/// Format: "sec min hour day month weekday"
const CLEANUP_SCHEDULE: &str = "0 0 4 * * 6";

/// Create and start the cron scheduler
pub async fn start_scheduler(
    db_pool: DbPool,
    config: &'static WorkerConfig,
) -> Result<JobScheduler, JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    let timezone: Tz = config.cron_timezone.parse().unwrap_or_else(|_| {
        tracing::warn!(
            timezone = %config.cron_timezone,
            "Invalid timezone, falling back to UTC"
        );
        chrono_tz::UTC
    });

    // Register cleanup job
    tracing::info!(
        schedule = CLEANUP_SCHEDULE,
        timezone = %timezone,
        "Registering cleanup cron job"
    );
    let cleanup_job = create_cleanup_job(db_pool.clone(), timezone)?;
    sched.add(cleanup_job).await?;

    sched.start().await?;

    Ok(sched)
}

fn create_cleanup_job(db_pool: DbPool, timezone: Tz) -> Result<Job, JobSchedulerError> {
    let db = Arc::clone(&db_pool);

    JobBuilder::new()
        .with_timezone(timezone)
        .with_cron_job_type()
        .with_schedule(CLEANUP_SCHEDULE)?
        .with_run_async(Box::new(move |_uuid, _lock| {
            let db = Arc::clone(&db);
            Box::pin(async move {
                cleanup::run_cleanup(&db).await;
            })
        }))
        .build()
}
