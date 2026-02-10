use crate::jobs::WorkerContext;
use crate::nats::consumer::NatsConsumer;
use crate::nats::streams::{INDEX_POST_CONSUMER, INDEX_POST_STREAM};
use crate::{DbPool, SearchClient};
use mofumofu_entity::{hashtags, post_hashtags, posts, users};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexPostJob {
    pub post_id: Uuid,
    pub action: PostIndexAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostIndexAction {
    Index,
    Delete,
}

pub const POSTS_INDEX: &str = "posts";

/// Build a search document JSON from post model with related data
pub fn build_post_search_json(
    post: &posts::Model,
    hashtags: &[String],
    author_handle: &str,
    author_display_name: &str,
    author_profile_image: Option<&str>,
) -> JsonValue {
    json!({
        "id": post.id.to_string(),
        "user_id": post.user_id.to_string(),
        "author_handle": author_handle,
        "author_display_name": author_display_name,
        "author_profile_image": author_profile_image,
        "title": post.title,
        "summary": post.summary,
        "slug": post.slug,
        "thumbnail_image": post.thumbnail_image,
        "hashtags": hashtags,
        "like_count": post.like_count,
        "comment_count": post.comment_count,
        "view_count": post.view_count,
        "published_at": post.published_at.map(|t| t.timestamp()),
        "created_at": post.created_at.timestamp(),
    })
}

/// MeiliSearch index settings for posts
fn post_index_settings() -> meilisearch_sdk::settings::Settings {
    meilisearch_sdk::settings::Settings::new()
        .with_searchable_attributes([
            "title",
            "summary",
            "author_handle",
            "author_display_name",
            "hashtags",
        ])
        .with_filterable_attributes(["user_id", "hashtags", "published_at"])
        .with_sortable_attributes(["created_at", "like_count", "view_count", "comment_count"])
        .with_displayed_attributes([
            "id",
            "user_id",
            "author_handle",
            "author_display_name",
            "author_profile_image",
            "title",
            "summary",
            "slug",
            "thumbnail_image",
            "hashtags",
            "like_count",
            "comment_count",
            "view_count",
            "published_at",
            "created_at",
        ])
        .with_ranking_rules([
            "words",
            "typo",
            "proximity",
            "attribute",
            "sort",
            "exactness",
        ])
}

async fn handle_index_post(
    job: IndexPostJob,
    client: &SearchClient,
    db: &DbPool,
) -> Result<(), anyhow::Error> {
    tracing::info!(
        "Processing post index job: post_id={}, action={:?}",
        job.post_id,
        job.action
    );

    let index = client.index(POSTS_INDEX);

    ensure_index_settings(client).await?;

    match job.action {
        PostIndexAction::Index => {
            let post = posts::Entity::find_by_id(job.post_id)
                .one(db.as_ref())
                .await?
                .ok_or_else(|| anyhow::anyhow!("Post not found: {}", job.post_id))?;

            // Fetch author
            let user = users::Entity::find_by_id(post.user_id)
                .one(db.as_ref())
                .await?
                .ok_or_else(|| anyhow::anyhow!("User not found: {}", post.user_id))?;

            // Fetch hashtags
            let post_hashtag_rows = post_hashtags::Entity::find()
                .filter(post_hashtags::Column::PostId.eq(job.post_id))
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

            let search_doc = build_post_search_json(
                &post,
                &hashtag_names,
                &user.handle,
                &user.display_name,
                user.profile_image.as_deref(),
            );

            index.add_documents(&[search_doc], Some("id")).await?;
            tracing::info!("Post {} indexed successfully", job.post_id);
        }
        PostIndexAction::Delete => {
            index.delete_document(&job.post_id.to_string()).await?;
            tracing::info!("Post {} deleted from index", job.post_id);
        }
    }

    Ok(())
}

/// Ensure index exists with proper settings
pub async fn ensure_index_settings(
    client: &meilisearch_sdk::client::Client,
) -> Result<(), anyhow::Error> {
    let index = client.index(POSTS_INDEX);

    match index.get_stats().await {
        Ok(_) => Ok(()),
        Err(meilisearch_sdk::errors::Error::Meilisearch(ref e))
            if e.error_code == meilisearch_sdk::errors::ErrorCode::IndexNotFound =>
        {
            tracing::info!("Creating posts index...");
            let task = client.create_index(POSTS_INDEX, Some("id")).await?;
            task.wait_for_completion(client, None, None).await?;

            tracing::info!("Applying posts index settings...");
            let index = client.index(POSTS_INDEX);
            let task = index.set_settings(&post_index_settings()).await?;
            task.wait_for_completion(client, None, None).await?;

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// Run the post index consumer
pub async fn run_consumer(ctx: WorkerContext) -> anyhow::Result<()> {
    let meili_client = ctx.meili_client.clone();
    let db_pool = ctx.db_pool.clone();

    let consumer = NatsConsumer::new(
        ctx.jetstream.clone(),
        INDEX_POST_STREAM,
        INDEX_POST_CONSUMER,
        2,
    );

    consumer
        .run::<IndexPostJob, _, _>(move |job| {
            let client = meili_client.clone();
            let db = db_pool.clone();
            async move { handle_index_post(job, &client, &db).await }
        })
        .await
}
