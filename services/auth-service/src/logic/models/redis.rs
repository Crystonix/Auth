// src/logic/models/redis.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::logic::models::OAuthProvider;
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
	pub session_id: String,
	pub user_id: i32,
	pub username: String,
	pub provider_user_id: Option<String>,
	pub avatar: Option<String>,
	pub role: UserRole,
	pub provider: OAuthProvider,
	pub session_version: u32,
	pub created_at: NaiveDateTime,
	pub expires_at: NaiveDateTime,
	pub last_activity: NaiveDateTime,
	pub ip_address: Option<String>,
	pub user_agent: Option<String>,
}

impl UserSession {
	pub fn avatar_url(&self) -> Option<String> {
		match &self.avatar {
			Some(hash) if !hash.is_empty() => match self.provider {
				OAuthProvider::Discord => {
					// use provider_user_id (Discord ID) instead of internal user_id
					self.provider_user_id.as_ref().map(|id| {
						format!("https://cdn.discordapp.com/avatars/{}/{}.png", id, hash)
					})
				}
				OAuthProvider::Google => Some(hash.clone()), // Google avatar is usually a URL
				_ => Some(hash.clone()),
			},
			_ => match self.provider {
				OAuthProvider::Discord => {
					self.provider_user_id.as_ref().map(|id| {
						// fallback default avatar
						let discriminator: u32 = id.parse().unwrap_or(0);
						format!("https://cdn.discordapp.com/embed/avatars/{}.png", discriminator % 5)
					})
				}
				OAuthProvider::Google => Some("/images/default-google-avatar.png".into()),
				_ => None,
			},
		}
	}
}

impl UserSession {
	pub fn is_valid(&self) -> bool {
		self.user_id != 0
	}
}
