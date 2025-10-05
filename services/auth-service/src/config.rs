use dotenvy::dotenv;
use std::env;
use redis::aio::ConnectionManager;

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_redirect_uri: String,
    pub discord_auth_url: String,
    pub discord_token_url: String,
    pub frontend_url: String,
    pub token_encryption_key: [u8; 32],
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let key_str = env::var("TOKEN_ENCRYPTION_KEY").expect("TOKEN_ENCRYPTION_KEY required");
        let key_bytes: [u8; 32] = key_str.as_bytes().try_into().expect("Key must be 32 bytes");

        Self {
            port: env::var("PORT").expect("PORT must be set").parse().expect("PORT must be a number"),
            database_url: env::var("AUTH_DB_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            discord_client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set"),
            discord_client_secret: env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set"),
            discord_redirect_uri: env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI must be set"),
            frontend_url: env::var("FRONTEND_URL").expect("FRONTEND_URL must be set"),
            discord_auth_url: env::var("DISCORD_AUTH_URL").expect("DISCORD_AUTH_URL must be set"),
            discord_token_url: env::var("DISCORD_TOKEN_URL").expect("DISCORD_TOKEN_URL must be set"),
            token_encryption_key: key_bytes,
        }
    }
}
