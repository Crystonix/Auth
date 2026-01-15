use sqlx::{PgPool, Result};
use crate::logic::models::OAuthToken;

/// Store or rotate refresh token for a given provider account
pub async fn upsert_oauth_token(
	pool: &PgPool,
	user_provider_id: i32,
	refresh_token: Vec<u8>,
) -> Result<OAuthToken> {
	sqlx::query_as!(
        OAuthToken,
        r#"
        INSERT INTO oauth_tokens (user_provider_id, encrypted_refresh_token, previous_refresh_token, last_token_rotation)
        VALUES ($1, $2, NULL, NOW())
        ON CONFLICT (user_provider_id) DO UPDATE
        SET
            previous_refresh_token = oauth_tokens.encrypted_refresh_token,
            encrypted_refresh_token = EXCLUDED.encrypted_refresh_token,
            last_token_rotation = NOW(),
            updated_at = NOW()
        RETURNING
            id,
            user_provider_id,
            encrypted_refresh_token,
            previous_refresh_token,
            created_at,
            updated_at,
            last_token_rotation
        "#,
        user_provider_id,
        refresh_token
    )
		.fetch_one(pool)
		.await
}

/// Get refresh token for a provider account
pub async fn get_oauth_token(
	pool: &PgPool,
	user_provider_id: i32,
) -> Result<Option<OAuthToken>> {
	sqlx::query_as!(
        OAuthToken,
        r#"
        SELECT
            id,
            user_provider_id,
            encrypted_refresh_token,
            previous_refresh_token,
            created_at,
            updated_at,
            last_token_rotation
        FROM oauth_tokens
        WHERE user_provider_id = $1
        "#,
        user_provider_id
    )
		.fetch_optional(pool)
		.await
}

/// Delete refresh token (logout / revoke)
pub async fn delete_oauth_token(
	pool: &PgPool,
	user_provider_id: i32,
) -> Result<()> {
	sqlx::query!(
        r#"
        DELETE FROM oauth_tokens
        WHERE user_provider_id = $1
        "#,
        user_provider_id
    )
		.execute(pool)
		.await?;

	Ok(())
}
