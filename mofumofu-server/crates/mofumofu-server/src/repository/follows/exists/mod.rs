mod newer;
mod older;

pub use newer::{repository_exists_newer_follower, repository_exists_newer_following};
pub use older::{repository_exists_older_follower, repository_exists_older_following};
