use crate::entity::posts::Model as PostModel;
use crate::repository::post::find_post_by_handle_and_slug::repository_find_post_by_handle_and_slug;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;

pub async fn repository_get_post_by_handle_and_slug<C>(
    conn: &C,
    handle: &str,
    slug: &str,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait,
{
    repository_find_post_by_handle_and_slug(conn, handle, slug)
        .await?
        .ok_or(Errors::PostNotFound)
}
