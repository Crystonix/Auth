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
use crate::logic::models::{SessionUser, UserRole};

pub async fn me(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> (StatusCode, Json<SessionUser>) {
    // Get session_id from cookie
    let session_id = match jar.get("session_id") {
        Some(c) => c.value().to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(SessionUser {
            id: "".into(),
            username: "".into(),
            avatar: None,
            role: UserRole::User,
        })),
    };

    // Get Redis connection
    let mut con = state.redis_pool.get().await.unwrap();

    // Fetch session data
    let user_map: HashMap<String, String> =
      con.hgetall(format!("user_session:{}", session_id))
        .await
        .unwrap_or_default();

    // Session expired?
    if user_map.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(SessionUser {
            id: "".into(),
            username: "".into(),
            avatar: None,
            role: UserRole::User,
        }));
    }

    // Optionally extend TTL
    let _: () = con
      .expire(format!("user_session:{}", session_id), 30 * 24 * 3600)
      .await
      .unwrap_or(());

    // Map Redis hash to strongly typed struct
    let session_user = SessionUser {
        id: user_map.get("id").cloned().unwrap_or_default(),
        username: user_map.get("username").cloned().unwrap_or_default(),
        avatar: user_map.get("avatar").cloned(),
        role: match user_map.get("role").map(|r| r.as_str()) {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        },
    };

    (StatusCode::OK, Json(session_user))
}
