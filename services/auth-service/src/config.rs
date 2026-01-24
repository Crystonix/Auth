// src/config.rs
use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub auth_service_port: u16,
    pub auth_db_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_redirect_uri: String,
    pub discord_auth_url: String,
    pub discord_token_url: String,
    pub encryption_key: [u8; 32],
    pub is_production: bool,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        // Decode the Base64 key
        let key_str = std::env::var("TOKEN_ENCRYPTION_KEY").expect("TOKEN_ENCRYPTION_KEY required");
        let key_bytes_vec = hex::decode(&key_str)
            .expect("Failed to decode TOKEN_ENCRYPTION_KEY from HEX");
        let key_bytes: [u8; 32] = key_bytes_vec
            .try_into()
            .expect("Decoded key must be exactly 32 bytes");

        Self {
            auth_service_port: env::var("AUTH_SERVICE_PORT").expect("AUTH_SERVICE_PORT must be set").parse().expect("AUTH_SERVICE_PORT must be a number"),
            auth_db_port: env::var("AUTH_DB_PORT").expect("AUTH_DB_PORT must be set").parse().expect("AUTH_DB_PORT must be a number"),
            database_url: env::var("AUTH_DB_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            discord_client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set"),
            discord_client_secret: env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set"),
            discord_redirect_uri: env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI must be set"),
            discord_auth_url: env::var("DISCORD_AUTH_URL").expect("DISCORD_AUTH_URL must be set"),
            discord_token_url: env::var("DISCORD_TOKEN_URL").expect("DISCORD_TOKEN_URL must be set"),
            encryption_key: key_bytes,
            is_production: "production" == env::var("ENVIRONMENT").expect("ENVRONMENT must be set"),
            frontend_url: env::var("FRONTEND_URL").expect("FRONTEND_URL must be set"),
        }
    }
}
