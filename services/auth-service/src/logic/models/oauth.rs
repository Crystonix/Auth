use serde::Deserialize;

/// OAuth provider user info fetched from Discord/Google/etc.
#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUser {
	pub id: String,                      // provider-specific ID
	pub username: String,
	pub discriminator: Option<String>,
	pub avatar: Option<String>,
}