// src/handlers/callback.rs
use std::collections::HashMap;
use std::sync::Arc;

use crate::logic::models::postgres::{OAuthProvider, UserRole};
use crate::logic::models::{oauth::DiscordUser, OAuthSession, UserSession};
use crate::logic::oauth::create_oauth_client;
use crate::queries::oauth_tokens::upsert_oauth_token;
use crate::queries::redis::session::{delete_oauth_session, get_oauth_session, store_user_session};
use crate::queries::user_providers::upsert_user_provider;
use crate::queries::users::upsert_user;
use crate::AppState;
use anyhow::Result;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};
use reqwest::Client as ReqwestClient;

const PROVIDER_DISCORD: OAuthProvider = OAuthProvider::Discord;

pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let error_url = "/auth/error";

    let fail_redirect = |jar: CookieJar, error| async move {
        let jar = jar.remove(Cookie::from("session_id"));
        (jar, Redirect::to(&format!("{}?error={}", error_url, error)))
    };

    // 1) Validate OAuth session from cookie + Redis
    let (session_id, oauth_session) = match validate_oauth_session(&state, &jar).await {
        Ok(s) => s,
        Err(e) => return fail_redirect(jar, e).await.into_response(),
    };

    // 2) CSRF check
    if let Err(e) = verify_csrf(&oauth_session, params.get("state")) {
        return fail_redirect(jar, e).await.into_response();
    }

    // 3) Extract authorization code
    let code = match params.get("code") {
        Some(c) => AuthorizationCode::new(c.to_string()),
        None => return fail_redirect(jar, "missing_code").await.into_response(),
    };

    // 4) Exchange code for OAuth tokens
    let client = create_oauth_client(&state).await;
    let token = match client
      .exchange_code(code)
      .set_pkce_verifier(PkceCodeVerifier::new(oauth_session.pkce_verifier.clone()))
      .request_async(&state.oauth2_client)
      .await {
        Ok(t) => t,
        Err(_) => return fail_redirect(jar, error_url).await.into_response(),
    };

    let access_token = token.access_token().secret();
    let refresh_token_bytes = token
      .refresh_token()
      .map(|r| r.secret().as_bytes().to_vec());

    // 5) Fetch Discord user info
    let discord_user = match fetch_discord_user_info(&ReqwestClient::new(), access_token).await {
        Ok(u) => u,
        Err(e) => return fail_redirect(jar, e).await.into_response(),
    };

    // 6) Upsert user in DB
    let db_user = match upsert_user_record(&state, &discord_user).await {
        Ok(u) => u,
        Err(e) => return fail_redirect(jar, e).await.into_response(),
    };

    // 7) Upsert provider account
    if let Err(e) = upsert_provider_record(&state, db_user.id, &discord_user).await {
        return fail_redirect(jar, e).await.into_response();
    }

    // 8) Store refresh token in DB if present
    if let Some(rt) = refresh_token_bytes {
        if upsert_oauth_token(&state.db_pool, db_user.id, rt).await.is_err() {
            return fail_redirect(jar, "token_store_failed").await.into_response();
        }
    }

    // 9) Create ephemeral session in Redis
    if create_ephemeral_session(&state, db_user.id, &session_id, UserRole::User).await.is_err() {
        return fail_redirect(jar, "session_store_failed").await.into_response();
    }

    // 10) Delete OAuth session
    let _ = delete_oauth_session(&state.redis_client, &session_id).await;

    // 11) Set cookie + redirect
    let cookie = Cookie::build(("session_id", session_id.clone()))
      .path("/")
      .http_only(true)
      .secure(state.config.is_production)
      .same_site(axum_extra::extract::cookie::SameSite::Lax)
      .max_age(time::Duration::hours(24))
      .build();
    let jar = jar.add(cookie);

    let redirect_url = "/dashboard";

    (jar, Redirect::to(redirect_url)).into_response()
}

/////////////////////////
// Helper Functions
/////////////////////////

async fn validate_oauth_session(
    state: &AppState,
    jar: &CookieJar,
) -> Result<(String, OAuthSession), &'static str> {
    let session_id = jar.get("session_id")
      .map(|c| c.value().to_string())
      .ok_or("missing_session")?;

    let oauth_session = get_oauth_session(&state.redis_client, &session_id)
      .await
      .map_err(|_| "session_invalid")?
      .ok_or("session_invalid")?;

    Ok((session_id, oauth_session))
}


fn verify_csrf(oauth_session: &OAuthSession, returned_state: Option<&String>) -> Result<(), &'static str> {
    if let Some(state) = returned_state {
        if state != &oauth_session.csrf_token {
            return Err("csrf_mismatch");
        }
    }
    Ok(())
}


async fn fetch_discord_user_info(client: &ReqwestClient, token: &str) -> Result<DiscordUser, &'static str> {
    let resp = client.get("https://discord.com/api/users/@me")
      .bearer_auth(token)
      .send()
      .await
      .map_err(|_| "user_fetch_failed")?;

    let resp = resp.error_for_status().map_err(|_| "user_fetch_failed")?;
    resp.json::<DiscordUser>().await.map_err(|_| "user_fetch_failed")
}

async fn upsert_user_record(
    state: &AppState,
    discord_user: &DiscordUser,
) -> Result<crate::logic::models::postgres::User, &'static str> {
    let role = UserRole::User;
    let user_id = discord_user.id.parse::<i32>().unwrap_or_default();

    upsert_user(
        &state.db_pool,
        user_id,
        &discord_user.username,
        discord_user.avatar.as_deref(),
        role,
    ).await.map_err(|_| "user_upsert_failed")
}

async fn upsert_provider_record(
    state: &AppState,
    user_id: i32,
    discord_user: &DiscordUser,
) -> Result<(), &'static str> {
    upsert_user_provider(
        &state.db_pool,
        user_id,
        PROVIDER_DISCORD,
        &discord_user.id,
        discord_user.discriminator.as_deref(),
        discord_user.avatar.as_deref(),
        None,
    ).await.map_err(|_| "provider_upsert_failed")?;

    Ok(())
}

async fn create_ephemeral_session(
    state: &AppState,
    user_id: i32,
    session_id: &str,
    role: UserRole,
) -> Result<(), &'static str> {
    let now = chrono::Utc::now().naive_utc();
    let user_session = UserSession {
        session_id: session_id.to_string(),
        user_id,
        role,
        session_version: 1,
        created_at: now,
        expires_at: now + chrono::Duration::hours(24),
        last_activity: now,
        ip_address: None,
        user_agent: None,
    };

    store_user_session(&state.redis_client, session_id, &user_session, 24 * 3600)
      .await
      .map_err(|_| "session_store_failed")
}
