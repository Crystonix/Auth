mod config;
mod db;
mod models;
mod queries;
mod handlers;

use axum::{routing::get, routing::post, Router};
use std::sync::Arc;
use crate::handlers::commissions::*;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<sqlx::PgPool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = Arc::new(db::connect().await?);

    let state = AppState { db_pool };
    let addr = format!("0.0.0.0:{}", state.config.auth_service_port.clone());
    let state = Arc::new(state);

    let app = Router::new()
      .route("/commissions", get(get_commissions).post(create_commission))
      .with_state(state);

    // start server

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
