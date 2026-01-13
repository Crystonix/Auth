// src/handlers/me.rs
use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use redis::AsyncCommands;
use crate::AppState;
use crate::logic::models::*;

pub async fn me(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> (StatusCode, Json<SessionUser>) {
    // 1️⃣ Get session_id from cookie
    let session_id = match jar.get("session_id") {
        Some(c) => c.value().to_string(),
        None => return unauthorized(),
    };

    // 2️⃣ Get Redis connection
    let mut con = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => return unauthorized(),
    };

    // 3️⃣ Fetch session data from Redis
    let user_map: HashMap<String, String> = match con
      .hgetall(format!("user_session:{}", session_id))
      .await
    {
        Ok(map) => map,
        Err(_) => return unauthorized(),
    };

    // 4️⃣ Session expired?
    if user_map.is_empty() {
        return unauthorized();
    }

    // 5️⃣ Optionally extend TTL
    let _: Result<(), _> = con
      .expire(format!("user_session:{}", session_id), 30 * 24 * 3600)
      .await;

    // 6️⃣ Map Redis hash to strongly typed struct
    let session_user = SessionUser {
        id: user_map.get("user_id").cloned().unwrap_or_default(),
        username: user_map.get("username").cloned().unwrap_or_default(),
        avatar: user_map.get("avatar").cloned(),
        role: match user_map.get("role").map(|r| r.as_str()) {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        },
    };

    (StatusCode::OK, Json(session_user))
}

/// Helper for unauthorized response
fn unauthorized() -> (StatusCode, Json<SessionUser>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(SessionUser {
            id: "".into(),
            username: "".into(),
            avatar: None,
            role: UserRole::User,
        }),
    )
}
