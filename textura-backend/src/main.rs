use crate::api::v0::routes::routes::api_routes;
use crate::config::db_config::DbConfig;
use crate::database::connection::establish_connection;
use crate::middleware::cors::cors_layer;
use crate::state::AppState;
use axum::Router;
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tracing::info;

mod api;
mod config;
mod database;
mod dto;
mod entity;
mod middleware;
mod service;
mod state;
mod utils;

pub async fn run_server() -> anyhow::Result<()> {
    let conn = establish_connection().await;
    let server_url = format!(
        "{}:{}",
        &DbConfig::get().server_host,
        &DbConfig::get().server_port
    );
    let app = Router::new()
        .merge(api_routes())
        .layer(cors_layer())
        .layer(CompressionLayer::new())
        .with_state(AppState { conn });

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
    DbConfig::init();

    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt::init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::ERROR)
            .init();
    }

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}
