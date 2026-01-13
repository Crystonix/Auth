use sqlx::{PgPool, Result};
use crate::logic::models::OAuthToken;

/// Store or rotate refresh token
pub async fn upsert_oauth_token(
	pool: &PgPool,
	user_id: &str,
	provider: &str,
	refresh_token: Vec<u8>,
) -> Result<OAuthToken> {
	sqlx::query_as!(
        OAuthToken,
        r#"
        INSERT INTO oauth_tokens (user_id, provider, refresh_token)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, provider) DO UPDATE
        SET
            refresh_token = EXCLUDED.refresh_token,
            updated_at = NOW()
        RETURNING
            id,
            user_id,
            provider,
            refresh_token,
            created_at,
            updated_at
        "#,
        user_id,
        provider,
        refresh_token
    )
		.fetch_one(pool)
		.await
}

/// Get refresh token for a user
pub async fn get_oauth_token(
	pool: &PgPool,
	user_id: &str,
) -> Result<Option<OAuthToken>> {
	sqlx::query_as!(
        OAuthToken,
        r#"
        SELECT
            id,
            user_id,
            provider,
            refresh_token,
            created_at,
            updated_at
        FROM oauth_tokens
        WHERE user_id = $1
        "#,
        user_id
    )
		.fetch_optional(pool)
		.await
}

/// Delete refresh token (logout / revoke)
pub async fn delete_oauth_token(
	pool: &PgPool,
	user_id: &str,
) -> Result<()> {
	sqlx::query!(
        r#"
        DELETE FROM oauth_tokens
        WHERE user_id = $1
        "#,
        user_id
    )
		.execute(pool)
		.await?;

	Ok(())
}
