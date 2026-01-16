use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "oauth_provider")]
#[sqlx(rename_all = "lowercase")]
pub enum OAuthProvider {
	Discord,
	Google,
}

impl fmt::Display for OAuthProvider {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			OAuthProvider::Discord => write!(f, "discord"),
			OAuthProvider::Google => write!(f, "google"),
		}
	}
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub avatar: Option<String>,
	pub role: UserRole,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub last_login: Option<DateTime<Utc>>,
	pub login_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserProvider {
	pub id: i32,
	pub user_id: i32,
	pub provider: OAuthProvider,
	pub provider_user_id: String,
	pub discriminator: Option<String>,
	pub avatar: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthToken {
	pub id: i32,
	pub user_provider_id: i32,

	pub encrypted_refresh_token: Vec<u8>,
	pub refresh_token_nonce: Vec<u8>,

	pub previous_refresh_token: Option<Vec<u8>>,
	pub previous_refresh_token_nonce: Option<Vec<u8>>,

	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub last_token_rotation: Option<DateTime<Utc>>,
}

