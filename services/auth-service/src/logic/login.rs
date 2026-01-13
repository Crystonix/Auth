use axum::extract::State;
use axum::response::Redirect;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use oauth2::{
	CsrfToken, PkceCodeChallenge
	, Scope,
};
// src/handlers/login.rs
use std::sync::Arc;
use uuid::Uuid;

use crate::logic::oauth::create_oauth_client;
use crate::logic::session::store_oauth_session;
use crate::AppState;

/// Handles Discord OAuth2 login (redirect to Discord)
pub async fn login(
	State(state): State<Arc<AppState>>,
	jar: CookieJar,
) -> Result<(CookieJar, Redirect), anyhow::Error> {
	// 1) Generate PKCE, CSRF, session_id
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
	let csrf_token = CsrfToken::new_random();
	let session_id = Uuid::new_v4().to_string();

	// Store secrets in Redis
	store_oauth_session(
		&state.redis_client,
		&session_id,
		&crate::logic::session::OAuthSession {
			csrf_token: csrf_token.secret().to_string(),
			pkce_verifier: pkce_verifier.secret().to_string(),
			nonce: "".to_string(), // not needed for pure OAuth2
		},
		600, // 10 minutes expiry
	)
		.await?;

	// 2) Build OAuth2 client
	let client = create_oauth_client(&state).await;


	// 3) Build authorization URL
	let (auth_url, _csrf_token) = client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("identify".into()))
		.add_scope(Scope::new("email".into()))
		.set_pkce_challenge(pkce_challenge)
		.url();

	// 4) Set session cookie
	let cookie = Cookie::build(("session_id", session_id))
		.http_only(true)
		.secure(state.config.is_production)
		.same_site(axum_extra::extract::cookie::SameSite::Lax)
		.max_age(time::Duration::hours(1))
		.path("/")
		.build();

	Ok((jar.add(cookie), Redirect::to(auth_url.as_str())))
}
