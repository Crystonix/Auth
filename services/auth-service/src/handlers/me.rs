// src/handlers/me.rs
use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use redis::AsyncCommands;
use serde_json::json;
use crate::AppState;

pub async fn me(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> (StatusCode, Json<serde_json::Value>) {
    // 1️⃣ Get session_id from cookie
    let session_id = match jar.get("session_id") {
        Some(c) => c.value().to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "No session"})),
            );
        }
    };

    // 2️⃣ Get Redis connection
    let mut con = state.redis_pool.get().await.unwrap();

    // 3️⃣ Fetch session data
    let user: HashMap<String, String> = con
        .hgetall(format!("user_session:{}", session_id))
        .await
        .unwrap_or_default();

    // 4️⃣ Check if session exists
    if user.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Session expired"})),
        );
    }

    // 5️⃣ Optionally extend session TTL to keep it alive
    let _: () = con
        .expire(format!("user_session:{}", session_id), 30 * 24 * 3600) // 30 days
        .await
        .unwrap_or(());

    // 6️⃣ Return user info as JSON
    (StatusCode::OK, Json(json!(user)))
}