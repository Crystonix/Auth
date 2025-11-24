use axum::{
    extract::{Query, State},
    response::Redirect,
};
use oauth2::*;
use std::sync::Arc;
use std::collections::HashMap;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use oauth2::basic::*;
use uuid::Uuid;
use crate::AppState;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use rand::RngCore;
use redis::AsyncCommands;
use reqwest::header;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}

use std::fmt;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

pub async fn create_oauth_client(state: &Arc<AppState>) -> Client<BasicErrorResponse, BasicTokenResponse, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse, EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet> {
    let client = BasicClient::new(ClientId::new(state.config.discord_client_id.clone()))
        .set_client_secret(ClientSecret::new(state.config.discord_client_secret.clone()))
        .set_auth_uri(AuthUrl::new(state.config.discord_auth_url.clone()).expect("REASON"))
        .set_token_uri(TokenUrl::new(state.config.discord_token_url.clone()).expect("REASON"))
        .set_redirect_uri(RedirectUrl::new(state.config.discord_redirect_uri.clone()).expect("REASON"));
    client

}

pub async fn login_string(state: State<Arc<AppState>>, jar: CookieJar) -> (CookieJar, Redirect) {
    login::<String>(state, jar).await
}

pub async fn login<RV: redis::FromRedisValue>(State(state): State<Arc<AppState>>, jar: CookieJar) -> (CookieJar, Redirect) {
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
    con.hset::<&std::string::String, &str, std::string::String, RV>(&key, "csrf_token", csrf_token.secret().to_string()).await.unwrap();
    con.hset::<&std::string::String, &str, std::string::String, RV>(&key, "pkce_verifier", _pkce_verifier.secret().to_string()).await.unwrap();
    con.expire::<&std::string::String, RV>(&key, 600).await.unwrap();

    // Retrieve PKCE + CSRF
    let (pkce_verifier_str, csrf_token_str): (Option<String>, Option<String>) =
        con.hmget(&key, &["pkce_verifier", "csrf_token"]).await.unwrap();

    let cookie = Cookie::build(("session_id", session_id.clone()))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .path("/")
        .build();

    (jar.add(cookie), Redirect::to(auth_url.as_str()))
}

// Callback handler to exchange code for access token
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
    let (pkce_verifier_str, csrf_token_str): (Option<String>, Option<String>) =
        con.hmget(&key, &["pkce_verifier", "csrf_token"]).await.unwrap();


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
            let _: () = con.hset_multiple(&session_key, &[
                ("user_id", &user.id),
                ("username", &user.username),
                ("discriminator", &user.discriminator),
                ("role", &role.to_string()),
            ]).await.unwrap();

            // Set TTL for 24 hours
            let _: () = con.expire(&session_key, 24*3600).await.unwrap();


            format!("Welcome, {}#{}!", user.username, user.discriminator)
        }
        Err(err) => format!("OAuth2 exchange failed: {:?}", err),
    }
}

pub fn encrypt_token(key: &[u8; 32], refresh_token: &str) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes); // in production, generate per-token
    cipher.encrypt(nonce, refresh_token.as_bytes()).expect("encryption failed")
}

pub fn decrypt_token(key: &[u8; 32], encrypted: &[u8]) -> String {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(b"unique nonce12");
    String::from_utf8(cipher.decrypt(nonce, encrypted).expect("decryption failed")).unwrap()
}