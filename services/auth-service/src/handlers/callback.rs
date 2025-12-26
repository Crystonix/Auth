// src/handlers/callback.rs
use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};
use redis::AsyncCommands;
use crate::AppState;
use crate::logic::models::{DiscordUser, UserRole};
use crate::logic::oauth::create_oauth_client;

pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let frontend_url = &state.config.frontend_url;

    // 1. Get session_id from cookie
    let session_id = match jar.get("session_id").map(|c| c.value().to_string()) {
        Some(id) => id,
        None => {
            let jar = jar.remove(Cookie::from("session_id"));
            return (jar, Redirect::to(&format!("{}?error=missing_session", frontend_url))).into_response()
        }
    };

     // 2. Get OAuth code
    let code = match params.get("code") {
        Some(c) => AuthorizationCode::new(c.to_string()),
        None => {
            let jar = jar.remove(Cookie::from("session_id"));
            return (jar, Redirect::to(&format!("{}?error=missing_code", frontend_url))).into_response();
        }
    };

    let returned_csrf = params.get("state").cloned();

    // 3. Retrieve PKCE + CSRF from Redis
    let key = format!("oauth_session:{}", session_id);
    let mut con = state.redis_pool.get().await.expect("Failed to connect to Redis");
    let (pkce_verifier_str, csrf_token_str): (Option<String>, Option<String>) =
        con.hmget(&key, &["pkce_verifier", "csrf_token"]).await.expect("Failed to retrieve PKCE + CSRF from Redis");

    let pkce_verifier_str = match pkce_verifier_str {
        Some(v) => v,
        None => {
            let jar = jar.remove(Cookie::from("session_id"));
            return (jar, Redirect::to(&format!("{}?error=session_invalid", frontend_url))).into_response();
        }
    };

    let csrf_token_str = csrf_token_str.unwrap_or_default();

    // 4. Verify CSRF token
    if let Some(returned_csrf) = returned_csrf {
        if returned_csrf != csrf_token_str {
            return (jar, Redirect::to(&format!("{}?error=csrf_mismatch", frontend_url))).into_response();
        }
    }

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_str);

    // 5. Exchange code for access + refresh token
    let client = create_oauth_client(&state).await;

    let token_result = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&state.http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();
            let refresh_token = token.refresh_token().map(|r| r.secret().to_string());

            // --- Step 6: Fetch Discord user profile ---
            let user: DiscordUser = state
                .http_client
                .get("https://discord.com/api/users/@me")
                .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
                .send()
                .await
                .expect("Failed to get Discord User")
                .json()
                .await
                .expect("Failed to get Discord User JSON");

            let role = UserRole::User; // default role

            // --- Step 7: Save user + tokens in Postgres ---
            sqlx::query!(
                r#"
                    INSERT INTO users (id, username, discriminator, avatar, role)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (id) DO UPDATE
                        SET username = EXCLUDED.username,
                            discriminator = EXCLUDED.discriminator,
                            avatar = EXCLUDED.avatar,
                            role = EXCLUDED.role
                    "#,
                user.id,
                user.username,
                user.discriminator,
                user.avatar,
                role.clone() as UserRole,
            )
                .execute(&*state.db_pool)
                .await
                .expect("Failed to save user + token in Postgres");

            if let Some(rt) = refresh_token {
                sqlx::query!(
                    r#"
            INSERT INTO oauth_tokens (user_id, refresh_token)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO UPDATE
                SET refresh_token = EXCLUDED.refresh_token
            "#,
                    user.id,
                    rt
                )
                    .execute(&*state.db_pool)
                    .await
                    .expect("Failed to set refresh token");
            }

            let session_key = format!("user_session:{}", session_id);
            let _: () = con
                .hset_multiple(
                    &session_key,
                    &[
                        ("user_id", &user.id),
                        ("username", &user.username),
                        ("discriminator", &user.discriminator),
                        ("role", &role.to_string()),
                    ],
                )
                .await
                .expect("Failed to create Session");

            // Set TTL for 24 hours
            let _: () = con.expire(&session_key, 24 * 3600).await.expect("Failed to set TTL");

            let cookie = Cookie::build(("session_id", session_id.clone()))
                .path("/")
                .http_only(true)
                .secure(true) // true in production
                .same_site(axum_extra::extract::cookie::SameSite::Lax)
                .max_age(time::Duration::hours(24))
                .build();

            let jar = jar.add(cookie);

            (jar, Redirect::to(&frontend_url)).into_response()


            // format!("Welcome, {}#{}!", user.username, user.discriminator)
        }
        Err(_) => {
            let jar = jar.remove(Cookie::from("session_id"));
            (jar, Redirect::to(&format!("{}?error=oauth_failed", frontend_url))).into_response()
        }
    }
}