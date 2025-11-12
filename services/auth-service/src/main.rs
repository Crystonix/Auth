mod config;
mod db;
mod models;
mod routes;

use axum::{routing::get, Router, ServiceExt};
use config::Config;
use db::connect;
use oauth2::reqwest;
use redis::aio::ConnectionManager;
use std::sync::{Arc, Mutex};
use redis::Client;
use anyhow::{Result, Context};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: Arc<sqlx::PgPool>,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::from_env());
    let db_pool = Arc::new(connect(&config.database_url).await?);
    db::run_migrations(&db_pool).await?;
    let http_client = reqwest::Client::new();

    let redis_manager = RedisConnectionManager::new(config.redis_url.as_str())?;
    let redis_pool = Pool::builder().max_size(10).build(redis_manager).await?;

    let state = AppState {
        config,
        db_pool,
        redis_pool,
        http_client,
    };
    let state = Arc::new(state);

    let addr = format!("0.0.0.0:{}", state.config.auth_service_port.clone());

    let app = Router::new()
        .route("/", get(|| async { "Auth service running" }))
        .with_state(state);

    // start server

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
