use axum::{
	extract::{State, Path},
	http::StatusCode,
	Json,
};
use std::sync::Arc;
use crate::{AppState, queries::commissions as q, models::{Commission, SessionUser}};

pub async fn get_commissions(
	State(state): State<Arc<AppState>>,
	user: SessionUser,
) -> (StatusCode, Json<Vec<Commission>>) {
	let commissions = q::get_user_commissions(&state.db_pool, &user.id).await.unwrap_or_default();

	(StatusCode::OK)
}

pub async fn create_commission(
	State(state): State<Arc<AppState>>,
	user: SessionUser,
	Json(payload): Json<Commission>,
) -> StatusCode {

	let _ = q::create_commission(&state.db_pool, &payload).await;
	StatusCode::CREATED
}
