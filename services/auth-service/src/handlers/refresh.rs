// src/handlers/refresh.rs
use crate::logic::{
    crypto::{decrypt_token, encrypt_token},
    oauth::create_oauth_client,
};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use oauth2::{RefreshToken, TokenResponse};
use serde_json::json;
use std::sync::Arc;

use crate::queries::oauth_tokens::{get_oauth_token, upsert_oauth_token};
use crate::queries::redis::session::{get_user_session, store_user_session};

pub async fn refresh_session(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // 1️⃣ Get session_id from cookie
    let session_id = jar.get("session_id")
      .map(|c| c.value().to_string())
      .ok_or_else(|| (
          StatusCode::UNAUTHORIZED,
          Json(json!({"error": "No session"}))
      ))?;

    // 2️⃣ Load ephemeral session from Redis (for metadata)
    let mut session = get_user_session(&state.redis_client, &session_id)
      .await
      .map_err(|_| (
          StatusCode::UNAUTHORIZED,
          Json(json!({"error": "Failed to load session"}))
      ))?
      .ok_or((
          StatusCode::UNAUTHORIZED,
          Json(json!({"error": "Session not found"}))
      ))?;

    // 3️⃣ Load refresh token from Postgres
    let token_row = get_oauth_token(&state.db_pool, session.user_id)
      .await
      .map_err(|_| (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(json!({"error": "Failed to load refresh token"}))
      ))?
      .ok_or((
          StatusCode::UNAUTHORIZED,
          Json(json!({"error": "No refresh token available"}))
      ))?;

    // 4️⃣ Decrypt refresh token
    let refresh_token_str = decrypt_token(
        &state.config.encryption_key,
        &token_row.encrypted_refresh_token,
        &token_row.refresh_token_nonce,
    ).map_err(|_| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": "Failed to decrypt refresh token"}))
    ))?;

    // 5️⃣ Exchange refresh token for new access & refresh tokens
    let client = create_oauth_client(&state).await;
    let token = client.exchange_refresh_token(&RefreshToken::new(refresh_token_str))
      .request_async(&state.oauth2_client)
      .await
      .map_err(|_| (
          StatusCode::UNAUTHORIZED,
          Json(json!({"error": "Failed to refresh token"}))
      ))?;

    // 6️⃣ Encrypt new refresh token if present
    if let Some(new_rt) = token.refresh_token() {
        let (encrypted_rt, nonce) = encrypt_token(&state.config.encryption_key, new_rt.secret())
          .map_err(|_| (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(json!({"error": "Failed to encrypt refresh token"}))
          ))?;

        // 7️⃣ Upsert token into Postgres
        upsert_oauth_token(
            &state.db_pool,
            session.user_id,
            encrypted_rt,
            nonce,
        )
          .await
          .map_err(|_| (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(json!({"error": "Failed to store refreshed token"}))
          ))?;
    }

    // 8️⃣ Extend session TTL in Redis
    store_user_session(&state.redis_client, &session_id, &session, 30 * 24 * 3600)
      .await
      .map_err(|_| (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(json!({"error": "Failed to store session"}))
      ))?;

    // 9️⃣ Return new access token info
    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Session refreshed",
            "access_token": token.access_token().secret(),
            "expires_in": token.expires_in().map(|d| d.as_secs())
        })),
    ))
}
