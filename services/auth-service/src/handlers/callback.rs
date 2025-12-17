use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::header;
use axum_extra::extract::CookieJar;
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};
use redis::AsyncCommands;
use crate::AppState;
use crate::logic::models::{DiscordUser, UserRole};
use crate::logic::oauth::create_oauth_client;

pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
) -> String {
    // 1. Extract the `code` and `state`
    let code = match params.get("code") {
        Some(c) => AuthorizationCode::new(c.to_string()),
        None => return "Missing authorization code".into(),
    };

    let returned_csrf = params.get("state").cloned();

    // 2. Get session_id from cookie
    let session_cookie = jar.get("session_id");
    let Some(session_id) = session_cookie.map(|c| c.value().to_string()) else {
        return "Missing session cookie".into();
    };

    let key = format!("oauth_session:{}", session_id);
    let mut con = state.redis_pool.get().await.unwrap(); // get a pooled connection

    // Retrieve PKCE + CSRF
    let (pkce_verifier_str, csrf_token_str): (Option<String>, Option<String>) = con
        .hmget(&key, &["pkce_verifier", "csrf_token"])
        .await
        .unwrap();

    let pkce_verifier_str = match pkce_verifier_str {
        Some(v) => v,
        None => return "Session expired or invalid".into(),
    };

    let csrf_token_str = csrf_token_str.unwrap_or_default();

    // 4. Verify CSRF token
    if let Some(returned_csrf) = returned_csrf {
        if returned_csrf != csrf_token_str {
            return "CSRF token mismatch".into();
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
                .unwrap()
                .json()
                .await
                .unwrap();

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
                .unwrap();

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
                    .unwrap();
            }

            let key = format!("oauth_session:{}", session_id);
            let mut con = state.redis_pool.get().await.unwrap();

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
                .unwrap();

            // Set TTL for 24 hours
            let _: () = con.expire(&session_key, 24 * 3600).await.unwrap();

            format!("Welcome, {}#{}!", user.username, user.discriminator)
        }
        Err(err) => format!("OAuth2 exchange failed: {:?}", err),
    }
}