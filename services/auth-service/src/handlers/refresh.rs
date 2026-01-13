// src/handlers/refresh.rs
use crate::logic::{crypto::{decrypt_token, encrypt_token}, oauth::create_oauth_client};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use oauth2::{RefreshToken, TokenResponse};
use serde_json::json;
use std::sync::Arc;
use anyhow::Result;

use crate::logic::session::{get_user_session, store_user_session, UserSession};

pub async fn refresh_session(
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
            )
        }
    };

    // 2️⃣ Load session from Redis
    let mut session = match get_user_session(&state.redis_client, &session_id).await {
        Ok(Some(s)) => s,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Session not found"})),
            )
        }
    };

    // 3️⃣ Ensure refresh token exists
    let (encrypted_rt, nonce) = match (&session.refresh_token, &session.nonce) {
        (Some(rt), Some(n)) => (rt.clone(), *n),
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "No refresh token available"})),
            )
        }
    };

    // 4️⃣ Decrypt refresh token
    let refresh_token_str = match decrypt_token(&state.config.encryption_key, &encrypted_rt, &nonce) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to decrypt refresh token"})),
            )
        }
    };

    // 5️⃣ Exchange refresh token for new access & refresh tokens
    let client = create_oauth_client(&state).await;
    let token_result = client
      .exchange_refresh_token(&RefreshToken::new(refresh_token_str))
      .request_async(&state.oauth2_client)
      .await;

    let token = match token_result {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Failed to refresh token"})),
            )
        }
    };

    // 6️⃣ Encrypt new refresh token if present
    if let Some(new_rt) = token.refresh_token() {
        let (enc_rt, new_nonce) = encrypt_token(&state.config.encryption_key, new_rt.secret());
        session.refresh_token = Some(enc_rt);
        session.nonce = Some(new_nonce);
    }

    // 7️⃣ Update TTL and store session
    if store_user_session(&state.redis_client, &session_id, &session, 30 * 24 * 3600).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to store session"})),
        );
    }

    // 8️⃣ Respond success
    (
        StatusCode::OK,
        Json(json!({
            "message": "Session refreshed",
            "access_token": token.access_token().secret(),
            "expires_in": token.expires_in().map(|d| d.as_secs())
        })),
    )
}
