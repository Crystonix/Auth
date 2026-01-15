// src/queries/user_providers.rs
use sqlx::{PgPool, Result};
use chrono::Utc;
use serde_json::Value;
use crate::logic::models::postgres::{UserProvider, OAuthProvider};

/// Insert or update a user provider (OAuth account)
pub async fn upsert_user_provider(
	pool: &PgPool,
	user_id: i32,
	provider: OAuthProvider,
	provider_user_id: &str,
	discriminator: Option<&str>,
	avatar: Option<&str>,
	metadata: Option<Value>,
) -> Result<UserProvider> {
	sqlx::query_as::<_, UserProvider>(
		r#"
        INSERT INTO user_providers (
            user_id,
            provider,
            provider_user_id,
            discriminator,
            avatar,
            metadata
        )
        VALUES ($1, $2::oauth_provider, $3, $4, $5, $6)
        ON CONFLICT (provider, provider_user_id) DO UPDATE
        SET
            discriminator = EXCLUDED.discriminator,
            avatar = EXCLUDED.avatar,
            metadata = EXCLUDED.metadata,
            updated_at = NOW()
        RETURNING
            id,
            user_id,
            provider,
            provider_user_id,
            discriminator,
            avatar,
            created_at,
            updated_at
        "#
	)
		.bind(user_id)
		.bind(provider)             // Rust enum, works with runtime query
		.bind(provider_user_id)
		.bind(discriminator)
		.bind(avatar)
		.bind(metadata)
		.fetch_one(pool)
		.await
}
