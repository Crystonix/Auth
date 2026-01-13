use std::fmt;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use chrono::NaiveDateTime;

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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
	pub id: String,
	pub username: String,
	pub discriminator: String,
	pub avatar: Option<String>,
	pub role: UserRole,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthToken {
	pub id: i32,
	pub user_id: String,
	pub provider: String,
	pub refresh_token: Vec<u8>,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}
