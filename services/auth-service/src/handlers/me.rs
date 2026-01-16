// src/handlers/me.rs
use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use ::redis::AsyncCommands;
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

    // 3️⃣ Fetch session data from Redis (JSON)
    let key = format!("user_session:{}", session_id);
    let session_json: Option<String> = match con.get(&key).await {
        Ok(v) => v,
        Err(_) => return unauthorized(),
    };

    let session_json = match session_json {
        Some(v) => v,
        None => return unauthorized(),
    };

    // 4️⃣ Deserialize into UserSession
    let user_session: UserSession = match serde_json::from_str(&session_json) {
        Ok(s) => s,
        Err(_) => return unauthorized(),
    };

    // 5️⃣ Optionally extend TTL
    let _: Result<(), _> = con.expire(&key, 30 * 24 * 3600).await;

    // 6️⃣ Map to API struct
    let session_user = SessionUser {
        id: user_session.user_id,
        username: user_session.username,
        avatar: user_session.avatar,
        role: user_session.role,
    };

    (StatusCode::OK, Json(session_user))
}

/// Helper for unauthorized response
fn unauthorized() -> (StatusCode, Json<SessionUser>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(SessionUser {
            id: -1,
            username: "".into(),
            avatar: None,
            role: UserRole::User,
        }),
    )
}
