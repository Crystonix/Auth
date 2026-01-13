// src/handlers/callback.rs
use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};
use reqwest::Client as ReqwestClient;
use anyhow::Result;

use crate::logic::session::{
    delete_oauth_session, store_user_session, update_user_refresh_token, UserSession,
};
use crate::AppState;
use crate::logic::models::db::UserRole;
use crate::logic::models::DiscordUser;
use crate::logic::oauth::create_oauth_client;

pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let error_url = "/auth/error";

    async fn fail_redirect(jar: CookieJar, error_url: &str, error: &str) -> impl IntoResponse {
        let jar = jar.remove(Cookie::from("session_id"));
        (jar, Redirect::to(&format!("{}?error={}", error_url, error)))
    }

    let session_id = match jar.get("session_id").map(|c| c.value().to_string()) {
        Some(id) => id,
        None => return fail_redirect(jar, error_url, "missing_session").await.into_response(),
    };

    let code = match params.get("code") {
        Some(c) => AuthorizationCode::new(c.to_string()),
        None => return fail_redirect(jar, error_url, "missing_code").await.into_response(),
    };

    let oauth_session = match crate::logic::session::get_oauth_session(&state.redis_client, &session_id).await {
        Ok(Some(s)) => s,
        _ => return fail_redirect(jar, error_url, "session_invalid").await.into_response(),
    };

    // CSRF check
    if let Some(returned_state) = params.get("state") {
        if returned_state != &oauth_session.csrf_token {
            return fail_redirect(jar, error_url, "csrf_mismatch").await.into_response();
        }
    }

    let client = create_oauth_client(&state).await;

    // Exchange code for tokens
    let token = match client
      .exchange_code(code)
      .set_pkce_verifier(PkceCodeVerifier::new(oauth_session.pkce_verifier.clone()))
      .request_async(&state.oauth2_client)
      .await
    {
        Ok(t) => t,
        Err(_) => return fail_redirect(jar, error_url, "oauth_failed").await.into_response(),
    };

    let access_token = token.access_token().secret();
    let refresh_token_bytes = token
      .refresh_token()
      .map(|r| r.secret().as_bytes().to_vec());
    let nonce_bytes = refresh_token_bytes.as_ref().map(|_| rand::random::<[u8; 12]>());

    // Fetch Discord user info
    let http_client = ReqwestClient::new();
    let user = match fetch_discord_user(&http_client, access_token).await {
        Ok(u) => u,
        Err(_) => return fail_redirect(jar, error_url, "user_fetch_failed").await.into_response(),
    };

    // Create UserSession
    let role = UserRole::User;
    let mut user_session = UserSession {
        user_id: user.id.clone(),
        username: user.username.clone(),
        discriminator: user.discriminator.clone(),
        role: role.to_string(),
        refresh_token: refresh_token_bytes.clone(),
        nonce: nonce_bytes,
    };

    // Store or update refresh token atomically
    if let (Some(rt), Some(nonce)) = (refresh_token_bytes, nonce_bytes) {
        let _ = update_user_refresh_token(&state.redis_client, &session_id, rt, nonce, 30 * 24 * 3600).await;
    }

    // Fallback: store session as JSON if no refresh token
    if store_user_session(&state.redis_client, &session_id, &user_session, 24 * 3600).await.is_err() {
        return fail_redirect(jar, error_url, "session_store_failed").await.into_response();
    }

    let _ = delete_oauth_session(&state.redis_client, &session_id).await;

    // Set cookie + redirect
    let cookie = Cookie::build(("session_id", session_id.clone()))
      .path("/")
      .http_only(true)
      .secure(state.config.is_production)
      .same_site(axum_extra::extract::cookie::SameSite::Lax)
      .max_age(time::Duration::hours(24))
      .build();
    let jar = jar.add(cookie);

    let redirect_url = match role {
        UserRole::Admin => "/admin",
        _ => "/dashboard",
    };

    (jar, Redirect::to(redirect_url)).into_response()
}

async fn fetch_discord_user(client: &ReqwestClient, token: &str) -> Result<DiscordUser> {
    let resp = client
      .get("https://discord.com/api/users/@me")
      .bearer_auth(token)
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Discord request failed: {e}"))?;

    let resp = resp.error_for_status()
      .map_err(|e| anyhow::anyhow!("Discord response error: {e}"))?;

    let user = resp.json::<DiscordUser>().await
      .map_err(|e| anyhow::anyhow!("Discord JSON parse error: {e}"))?;

    Ok(user)
}
