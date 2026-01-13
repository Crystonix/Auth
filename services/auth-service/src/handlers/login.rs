// src/handlers/login.rs
use crate::logic::login::login;
use crate::AppState;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;

pub async fn login_handler(
    state: State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    match login(state, jar).await {
        Ok((jar, redirect)) => (jar, redirect).into_response(),
        Err(err) => {
            // Simple error response; you can improve this with a nicer HTML page or JSON
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Login error: {}", err)).into_response()
        }
    }
}
