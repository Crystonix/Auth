// src/main.rs
mod config;
mod db;
mod logic;
mod handlers;

use anyhow::Result;
use axum::{routing::get, Router};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use config::Config;
use db::connect;
use oauth2::reqwest;
use std::sync::Arc;
use std::time::Duration;
use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::*;

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

    let cors = CorsLayer::new()
      .allow_origin("http://localhost:5173".parse::<HeaderValue>()?) // frontend origin
      .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
      .allow_headers(Any)
      .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/", get(|| async { "Auth service running" }))
        .route("/auth/discord/login", get(login_string))
        .route("/auth/discord/callback", get(callback))
        .route("/auth/me", get(me))
        .route("/auth/refresh", get(refresh_session))
        .route("/auth/logout", get(logout))
        .layer(cors)
        .with_state(state);

    // start server

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
