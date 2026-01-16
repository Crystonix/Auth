// src/logic/models/redis.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::logic::models::postgres::UserRole;

#[derive(Serialize, Deserialize, Clone)]
pub struct OAuthSession {
	pub csrf_token: String,
	pub pkce_verifier: String,
	pub nonce: String,
}


/// Session representation for API / frontend
#[derive(Debug, Serialize, Clone)]
pub struct SessionUser {
	pub id: i32,                        // internal user ID
	pub username: String,
	pub avatar: Option<String>,
	pub role: UserRole,
}

/// Ephemeral user session stored in Redis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
	pub session_id: String,              // UUID as string
	pub user_id: i32,
	pub username: String,                // internal username
	pub avatar: Option<String>,          // optional avatar URL// internal user ID
	pub role: UserRole,
	pub session_version: u32,            // incremental session version
	pub created_at: NaiveDateTime,
	pub expires_at: NaiveDateTime,
	pub last_activity: NaiveDateTime,
	pub ip_address: Option<String>,
	pub user_agent: Option<String>,
}

impl UserSession {
	pub fn is_valid(&self) -> bool {
		self.user_id != 0
	}
}
