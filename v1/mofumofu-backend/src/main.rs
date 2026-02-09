use crate::api::v0::routes::routes::api_routes;
use crate::config::db_config::DbConfig;
use crate::connection::cloudflare_r2::establish_r2_connection;
use crate::connection::database::establish_connection;
use crate::connection::http::create_http_client;
use crate::connection::meilisearch::MeilisearchClient;
use crate::connection::redis_connection::establish_redis_connection;
use crate::middleware::cors::cors_layer;
use crate::state::AppState;
use crate::utils::logger::init_tracing;
use axum::Router;
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tracing::{error, info};

mod api;
mod config;
mod connection;
mod dto;
mod entity;
mod microservices;
mod middleware;
mod repository;
mod service;
mod state;
mod utils;

pub async fn run_server() -> anyhow::Result<()> {
    let conn = establish_connection().await;
    let cloudflare_r2 = establish_r2_connection().await.map_err(|e| {
        error!("Failed to establish cloudflare_r2 connection: {}", e);
        anyhow::anyhow!("R2 connection failed: {}", e)
    })?;
    let redis = establish_redis_connection().await.map_err(|e| {
        error!("Failed to establish redis connection: {}", e);
        anyhow::anyhow!("Redis connection failed: {}", e)
    })?;
    let http_client = create_http_client().await.map_err(|e| {
        error!("Failed to create HTTP client: {}", e);
        anyhow::anyhow!("HTTP client creation failed: {}", e)
    })?;

    let meilisearch = MeilisearchClient::new().map_err(|e| {
        error!("Failed to create Meilisearch client: {}", e);
        anyhow::anyhow!("Meilisearch client creation failed: {}", e)
    })?;

    // Meilisearch 인덱스 설정
    if let Err(e) = crate::service::meilisearch::post_indexer::setup_posts_index(&meilisearch).await
    {
        error!("Failed to setup Meilisearch posts index: {}", e);
    } else {
        info!("Meilisearch posts index setup completed");
    }

    let server_url = format!(
        "{}:{}",
        &DbConfig::get().server_host,
        &DbConfig::get().server_port
    );
    let app = Router::new()
        .merge(api_routes())
        .layer(cors_layer())
        .layer(CompressionLayer::new())
        .with_state(AppState {
            conn,
            cloudflare_r2,
            redis,
            http_client,
            meilisearch,
        });

    info!("Starting server at: {}", server_url);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // tracing 초기화
    init_tracing();

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}
