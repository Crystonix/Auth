use serde::{Deserialize, Serialize};
use crate::logic::models::db::UserRole;

/// Session/User API types
#[derive(Debug, Serialize, Clone)]
pub struct SessionUser {
	pub id: String,
	pub username: String,
	pub avatar: Option<String>,
	pub role: UserRole,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUser {
	pub id: String,
	pub username: String,
	pub discriminator: String,
	pub avatar: Option<String>,
}
