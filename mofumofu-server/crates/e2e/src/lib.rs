//! E2E test framework for mofumofu.
//!
//! Provides test infrastructure using Docker containers for PostgreSQL, Redis, MeiliSearch,
//! NATS, and the mofumofu server/worker.

pub mod fixtures;
pub mod helpers;

pub use fixtures::TestServer;
pub use helpers::TestUser;
