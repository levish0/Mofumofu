use super::common::DEFAULT_BATCH_SIZE;
use super::{ReindexJobBase, ReindexPostsJob};
use crate::jobs::WorkerContext;
use crate::jobs::index::post::{POSTS_INDEX, build_post_search_json, ensure_index_settings};
use crate::nats::JetStreamContext;
use crate::nats::consumer::NatsConsumer;
use crate::nats::publisher::publish_job;
use crate::nats::streams::{REINDEX_POSTS_CONSUMER, REINDEX_POSTS_STREAM, REINDEX_POSTS_SUBJECT};
use crate::{DbPool, SearchClient};
use mofumofu_entity::{hashtags, post_hashtags, posts, users};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

/// Handle a batch reindex job for posts
async fn handle_reindex_posts(
    job: ReindexPostsJob,
    client: &SearchClient,
    db: &DbPool,
    jetstream: &JetStreamContext,
) -> Result<(), anyhow::Error> {
    tracing::info!(
        reindex_id = %job.base.reindex_id,
        batch_number = job.base.batch_number,
        after_id = ?job.base.after_id,
        batch_size = job.base.batch_size,
        "Processing post reindex batch"
    );

    // First batch: ensure index settings and clear existing data
    if job.base.after_id.is_none() {
        ensure_index_settings(client).await?;

        let index = client.index(POSTS_INDEX);
        index.delete_all_documents().await?;

        let total = posts::Entity::find().count(db.as_ref()).await?;
        tracing::info!(
            reindex_id = %job.base.reindex_id,
            total_posts = total,
            "Starting post reindex"
        );
    }

    // Fetch batch of posts
    let posts_batch =
        fetch_posts_batch(db.as_ref(), job.base.after_id, job.base.batch_size).await?;

    if posts_batch.is_empty() {
        tracing::info!(
            reindex_id = %job.base.reindex_id,
            total_batches = job.base.batch_number,
            "Post reindex completed"
        );
        return Ok(());
    }

    // Build search documents (need author + hashtags for each post)
    let mut search_docs = Vec::with_capacity(posts_batch.len());
    for post in &posts_batch {
        let user = users::Entity::find_by_id(post.user_id)
            .one(db.as_ref())
            .await?;

        let (author_handle, author_display_name) = match user {
            Some(u) => (u.handle, u.display_name),
            None => {
                tracing::warn!(
                    "Skipping post {} — author {} not found",
                    post.id,
                    post.user_id
                );
                continue;
            }
        };

        let post_hashtag_rows = post_hashtags::Entity::find()
            .filter(post_hashtags::Column::PostId.eq(post.id))
            .all(db.as_ref())
            .await?;

        let mut hashtag_names = Vec::with_capacity(post_hashtag_rows.len());
        for ph in &post_hashtag_rows {
            if let Some(tag) = hashtags::Entity::find_by_id(ph.hashtag_id)
                .one(db.as_ref())
                .await?
            {
                hashtag_names.push(tag.name);
            }
        }

        search_docs.push(build_post_search_json(
            post,
            &hashtag_names,
            &author_handle,
            &author_display_name,
        ));
    }

    // Index batch to MeiliSearch
    let index = client.index(POSTS_INDEX);
    index.add_documents(&search_docs, Some("id")).await?;

    let processed_count = posts_batch.len();
    let last_id = posts_batch
        .last()
        .map(|p| p.id)
        .ok_or_else(|| anyhow::anyhow!("posts_batch unexpectedly empty"))?;

    tracing::info!(
        reindex_id = %job.base.reindex_id,
        batch_number = job.base.batch_number,
        processed = processed_count,
        last_id = %last_id,
        "Batch processed"
    );

    // Self-enqueue next batch via NATS
    let next_job = ReindexPostsJob {
        base: ReindexJobBase {
            after_id: Some(last_id),
            batch_size: job.base.batch_size,
            reindex_id: job.base.reindex_id,
            batch_number: job.base.batch_number + 1,
        },
    };

    publish_job(jetstream, REINDEX_POSTS_SUBJECT, &next_job).await?;

    Ok(())
}

/// Fetch a batch of posts using UUID v7 cursor pagination
async fn fetch_posts_batch(
    db: &sea_orm::DatabaseConnection,
    after_id: Option<Uuid>,
    batch_size: u32,
) -> Result<Vec<posts::Model>, anyhow::Error> {
    let mut query = posts::Entity::find().order_by_asc(posts::Column::Id);

    if let Some(cursor) = after_id {
        query = query.filter(posts::Column::Id.gt(cursor));
    }

    let posts = query.limit(batch_size as u64).all(db).await?;

    Ok(posts)
}

/// Create a new ReindexPostsJob to start reindexing from the beginning
pub fn create_reindex_posts_job(reindex_id: Uuid, batch_size: Option<u32>) -> ReindexPostsJob {
    ReindexPostsJob {
        base: ReindexJobBase {
            after_id: None,
            batch_size: batch_size.unwrap_or(DEFAULT_BATCH_SIZE),
            reindex_id,
            batch_number: 1,
        },
    }
}

/// Run the reindex posts consumer
pub async fn run_consumer(ctx: WorkerContext) -> anyhow::Result<()> {
    let meili_client = ctx.meili_client.clone();
    let db_pool = ctx.db_pool.clone();
    let jetstream = ctx.jetstream.clone();

    let consumer = NatsConsumer::new(
        ctx.jetstream.clone(),
        REINDEX_POSTS_STREAM,
        REINDEX_POSTS_CONSUMER,
        1,
    );

    consumer
        .run::<ReindexPostsJob, _, _>(move |job| {
            let client = meili_client.clone();
            let db = db_pool.clone();
            let js = jetstream.clone();
            async move { handle_reindex_posts(job, &client, &db, &js).await }
        })
        .await
}
