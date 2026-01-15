// src/main.rs
pub mod config;
pub mod db;
pub mod logic;
pub mod handlers;
pub mod queries;
use crate::handlers::*;
use anyhow::Result;
use axum::http::{HeaderValue, Method};
use axum::{routing::get, Router};
use config::Config;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use db::connect;
use crate::handlers::login::login_handler;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: Arc<PgPool>,
    pub redis_client: redis::Client,
    pub oauth2_client: oauth2::reqwest::Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::from_env());
    let db_pool = connect(&config.database_url).await?;
    db::run_migrations(&db_pool).await?;

    let oauth2_client = oauth2::reqwest::Client::new();

    let redis_client = redis::Client::open(config.redis_url.clone())?;

    let state = AppState {
        config: Arc::clone(&config),
        db_pool: Arc::new(db_pool),
        redis_client,
        oauth2_client,
    };
    let state = Arc::new(state);

    let addr = format!("0.0.0.0:{}", state.config.auth_service_port.clone());

    let cors = CorsLayer::new()
      .allow_origin(config.frontend_url.parse::<HeaderValue>()?)
      .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
      .allow_headers(Any)
      .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/", get(|| async { "Auth service running" }))
        .route("/discord/login", get(login_handler))
        .route("/discord/callback", get(callback))
        .route("/me", get(me))
        .route("/refresh", get(refresh_session))
        .route("/logout", get(logout))
        .layer(cors)
        .with_state(state);

    // start server

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
      .with_graceful_shutdown(async {
          signal::ctrl_c().await.expect("failed to listen to ctr-c");
          println!("Shutting down");
      })
      .await?;

    Ok(())
}
