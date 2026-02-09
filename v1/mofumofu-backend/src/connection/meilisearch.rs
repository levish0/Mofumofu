use crate::config::db_config::DbConfig;
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MeilisearchClient {
    pub client: Client,
}

impl MeilisearchClient {
    pub fn new() -> Result<Self, meilisearch_sdk::errors::Error> {
        let config = DbConfig::get();

        let client = if let Some(api_key) = &config.meilisearch_api_key {
            Client::new(&config.meilisearch_host, Some(api_key.as_str()))?
        } else {
            Client::new(&config.meilisearch_host, None::<&str>)?
        };

        Ok(MeilisearchClient { client })
    }

    pub fn get_client(&self) -> &meilisearch_sdk::client::Client {
        &self.client
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeilisearchPost {
    pub id: String,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub user_id: String,
    pub user_handle: String,
    pub user_name: String,
    pub hashtags: Vec<String>,
    pub created_at: i64, // Unix timestamp for filtering/sorting
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeilisearchPostId {
    pub id: String,
}

impl MeilisearchPost {
    pub fn from_post_with_user_and_hashtags(
        post: &crate::entity::posts::Model,
        user: &crate::entity::users::Model,
        hashtags: Vec<String>,
    ) -> Self {
        MeilisearchPost {
            id: post.id.to_string(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            content: post.content.clone(),
            user_id: post.user_id.to_string(),
            user_handle: user.handle.clone(),
            user_name: user.name.clone(),
            hashtags,
            created_at: post.created_at.timestamp(),
            like_count: post.like_count,
            comment_count: post.comment_count,
            view_count: post.view_count,
        }
    }
}
