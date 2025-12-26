// src/handlers/login.rs
use std::sync::Arc;
use axum::extract::State;
use axum::response::Redirect;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use uuid::Uuid;
use crate::AppState;
use crate::logic::oauth::create_oauth_client;

pub async fn login_string(state: State<Arc<AppState>>, jar: CookieJar) -> (CookieJar, Redirect) {
    login::<String>(state, jar).await
}

pub async fn login<RV: redis::FromRedisValue>(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> (CookieJar, Redirect) {
    let client = create_oauth_client(&state).await;
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let session_id = Uuid::new_v4().to_string();

    use redis::AsyncCommands;

    let mut con = state.redis_pool.get().await.unwrap();

    // Store PKCE + CSRF
    let key = format!("oauth_session:{}", session_id);
    con.hset::<&String, &str, String, RV>(
        &key,
        "csrf_token",
        csrf_token.secret().to_string(),
    )
        .await
        .unwrap();
    con.hset::<&String, &str, String, RV>(
        &key,
        "pkce_verifier",
        _pkce_verifier.secret().to_string(),
    )
        .await
        .unwrap();
    con.expire::<&String, RV>(&key, 600)
        .await
        .unwrap();

    // Retrieve PKCE + CSRF
    let (pkce_verifier_str, csrf_token_str): (Option<String>, Option<String>) = con
        .hmget(&key, &["pkce_verifier", "csrf_token"])
        .await
        .unwrap();

    let cookie = Cookie::build(("session_id", session_id.clone()))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(time::Duration::hours(1))
        .path("/")
        .build();

    (jar.add(cookie), Redirect::to(auth_url.as_str()))
}