use std::sync::Arc;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use redis::AsyncCommands; // Async trait needed
use crate::AppState;

pub async fn logout(
	State(state): State<Arc<AppState>>,
	jar: CookieJar,
) -> impl IntoResponse {
	// 1) Read session cookie (if present)
	if let Some(cookie) = jar.get("session_id") {
		let session_id = cookie.value().to_string();

		// 2) Delete Redis session (best‑effort)
		if let Ok(mut con) = state.redis_client.get_multiplexed_async_connection().await {
			// Delete the session key (ignore result)
			let _ : redis::RedisResult<()> =
				con.del(format!("user_session:{}", session_id)).await;
		}
	}

	// 3) Remove cookie from client
	let jar = jar.remove(Cookie::named("session_id"));

	// 4) Redirect to home
	(jar, Redirect::to("/"))
}
