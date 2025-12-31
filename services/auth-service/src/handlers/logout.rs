// src/handlers/logout.rs

use std::sync::Arc;
use axum::{
	extract::State,
};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use redis::AsyncCommands;
use crate::AppState;

pub async fn logout(
	State(state): State<Arc<AppState>>,
	jar: CookieJar,
) -> impl IntoResponse {
	// 1️⃣ Read session cookie (if present)
	if let Some(cookie) = jar.get("session_id") {
		let session_id = cookie.value();

		// 2️⃣ Delete Redis session (best-effort)
		if let Ok(mut con) = state.redis_pool.get().await {
			let _: redis::RedisResult<()> =
				con.del(format!("user_session:{}", session_id)).await;
		}
	}

	// 3️⃣ Remove cookie from client
	let jar = jar.remove("session_id");

	// 4️⃣ Return success regardless of state
	(jar, Redirect::to("/"))
}
