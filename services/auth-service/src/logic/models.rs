use std::fmt;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) avatar: Option<String>,
}

#[derive(Debug, Clone, sqlx::Type)]
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
