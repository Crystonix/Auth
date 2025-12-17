use crate::logic::crypto::{decrypt_token, encrypt_token};
use crate::logic::oauth::create_oauth_client;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use oauth2::{RefreshToken, TokenResponse};
use redis::AsyncCommands;
use serde_json::json;
use std::sync::Arc;

pub async fn refresh_session(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> (StatusCode, Json<serde_json::Value>) {
    let session_id = match jar.get("session_id") {
        Some(c) => c.value().to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "No session"})),
            );
        }
    };

    let mut con = state.redis_pool.get().await.unwrap();

    // Get refresh token + nonce
    let (encrypted_rt, nonce): (Vec<u8>, [u8; 12]) = con
        .hmget(
            format!("user_session:{}", session_id),
            &["refresh_token", "nonce"],
        )
        .await
        .unwrap();

    let refresh_token = decrypt_token(&state.config.encryption_key, &encrypted_rt, &nonce);

    let client = create_oauth_client(&state).await;
    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(&state.http_client)
        .await;

    match token_result {
        Ok(token) => {
            // Update Redis with new refresh token
            if let Some(rt) = token.refresh_token() {
                let (enc_rt, nonce_bytes) = encrypt_token(&state.config.encryption_key, rt.secret());
                let _: () = con
                    .hset_multiple(
                        format!("user_session:{}", session_id),
                        &[("refresh_token", enc_rt), ("nonce", Vec::from(nonce_bytes))],
                    )
                    .await
                    .unwrap();
            }

            (
                StatusCode::OK,
                Json(json!({"message": "Session refreshed"})),
            )
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Failed to refresh"})),
        ),
    }
}
