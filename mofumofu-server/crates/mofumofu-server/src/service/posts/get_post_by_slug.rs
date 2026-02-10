use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::repository_find_post_by_user_id_and_slug;
use crate::repository::user::repository_find_user_by_handle;
use crate::utils::r2_url::build_r2_public_url;
use crate::utils::redis_cache::{get_json_compressed, set_json_compressed};
use mofumofu_constants::{post_render_key, POST_RENDER_CACHE_TTL_SECONDS};
use mofumofu_dto::posts::{CachedPostRender, PostAuthor, PostResponse};
use mofumofu_entity::hashtags::Entity as HashtagEntity;
use mofumofu_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn service_get_post_by_slug(
    conn: &DatabaseConnection,
    redis_cache: &RedisClient,
    handle: String,
    slug: String,
) -> ServiceResult<PostResponse> {
    let user = repository_find_user_by_handle(conn, handle)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let post = repository_find_post_by_user_id_and_slug(conn, user.id, slug)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let author = PostAuthor {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
    };

    let post_hashtags = repository_find_post_hashtags_by_post_id(conn, post.id).await?;
    let mut hashtag_names = Vec::with_capacity(post_hashtags.len());
    for ph in &post_hashtags {
        if let Some(hashtag) = HashtagEntity::find_by_id(ph.hashtag_id).one(conn).await? {
            hashtag_names.push(hashtag.name);
        }
    }

    // Try cache for rendered content
    let cache_key = post_render_key(post.id);
    let (render, toc) =
        match get_json_compressed::<CachedPostRender>(redis_cache, &cache_key).await {
            Ok(Some(cached)) => (Some(cached.render), Some(cached.toc)),
            _ => {
                // Cache miss — use DB values and cache them for next time
                if let (Some(render), Some(toc)) = (&post.render, &post.toc) {
                    let cached = CachedPostRender {
                        render: render.clone(),
                        toc: toc.clone(),
                    };
                    if let Err(e) = set_json_compressed(
                        redis_cache,
                        &cache_key,
                        &cached,
                        POST_RENDER_CACHE_TTL_SECONDS,
                    )
                    .await
                    {
                        tracing::warn!(
                            "Failed to backfill render cache for {}: {:?}",
                            post.id,
                            e
                        );
                    }
                }
                (post.render.clone(), post.toc.clone())
            }
        };

    Ok(PostResponse {
        id: post.id,
        user_id: post.user_id,
        author,
        title: post.title,
        slug: post.slug,
        thumbnail_image: post.thumbnail_image,
        summary: post.summary,
        content: post.content,
        render,
        toc,
        like_count: post.like_count,
        comment_count: post.comment_count,
        view_count: post.view_count,
        hashtags: hashtag_names,
        published_at: post.published_at,
        created_at: post.created_at,
        updated_at: post.updated_at,
    })
}
