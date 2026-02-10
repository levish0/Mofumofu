use async_nats::jetstream::{
    Context as JetStream,
    stream::{Config as StreamConfig, RetentionPolicy},
};

// Stream names
pub const EMAIL_STREAM: &str = "mofumofu_jobs_email";
pub const INDEX_USER_STREAM: &str = "mofumofu_jobs_index_user";
pub const INDEX_POST_STREAM: &str = "mofumofu_jobs_index_post";
pub const REINDEX_USERS_STREAM: &str = "mofumofu_jobs_reindex_users";
pub const REINDEX_POSTS_STREAM: &str = "mofumofu_jobs_reindex_posts";

// Subjects (for publishing)
pub const EMAIL_SUBJECT: &str = "mofumofu.jobs.email";
pub const INDEX_USER_SUBJECT: &str = "mofumofu.jobs.index.user";
pub const INDEX_POST_SUBJECT: &str = "mofumofu.jobs.index.post";
pub const REINDEX_USERS_SUBJECT: &str = "mofumofu.jobs.reindex.users";
pub const REINDEX_POSTS_SUBJECT: &str = "mofumofu.jobs.reindex.posts";

// Consumer names
pub const EMAIL_CONSUMER: &str = "email-consumer";
pub const INDEX_USER_CONSUMER: &str = "user-index-consumer";
pub const INDEX_POST_CONSUMER: &str = "post-index-consumer";
pub const REINDEX_USERS_CONSUMER: &str = "reindex-users-consumer";
pub const REINDEX_POSTS_CONSUMER: &str = "reindex-posts-consumer";

/// Stream and subject pairs for initialization
const STREAMS: &[(&str, &str)] = &[
    (EMAIL_STREAM, EMAIL_SUBJECT),
    (INDEX_USER_STREAM, INDEX_USER_SUBJECT),
    (INDEX_POST_STREAM, INDEX_POST_SUBJECT),
    (REINDEX_USERS_STREAM, REINDEX_USERS_SUBJECT),
    (REINDEX_POSTS_STREAM, REINDEX_POSTS_SUBJECT),
];

/// Initialize all job streams with WorkQueue retention policy
pub async fn initialize_all_streams(jetstream: &JetStream) -> anyhow::Result<()> {
    for (stream_name, subject) in STREAMS {
        jetstream
            .get_or_create_stream(StreamConfig {
                name: stream_name.to_string(),
                subjects: vec![subject.to_string()],
                retention: RetentionPolicy::WorkQueue,
                ..Default::default()
            })
            .await?;
        tracing::info!("Stream {} ready (subject: {})", stream_name, subject);
    }
    Ok(())
}
