use sqlx::{PgPool, Result};
use crate::logic::models::{User, UserRole};

/// Insert or update a user (Discord login upsert)
pub async fn upsert_user(
	pool: &PgPool,
	id: &str,
	username: &str,
	discriminator: &str,
	avatar: Option<&str>,
	role: UserRole,
) -> Result<User> {
	sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, discriminator, avatar, role)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET
            username = EXCLUDED.username,
            discriminator = EXCLUDED.discriminator,
            avatar = EXCLUDED.avatar,
            role = EXCLUDED.role,
            updated_at = NOW()
        RETURNING
            id,
            username,
            discriminator,
            avatar,
            role AS "role: UserRole",
            created_at,
            updated_at
        "#,
        id,
        username,
        discriminator,
        avatar,
        role as UserRole
    )
		.fetch_one(pool)
		.await
}

/// Fetch user by id
pub async fn get_user_by_id(pool: &PgPool, id: &str) -> Result<Option<User>> {
	sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            discriminator,
            avatar,
            role AS "role: UserRole",
            created_at,
            updated_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
		.fetch_optional(pool)
		.await
}

/// Update user role (admin promotion, etc.)
pub async fn update_user_role(
	pool: &PgPool,
	id: &str,
	role: UserRole,
) -> Result<()> {
	sqlx::query!(
        r#"
        UPDATE users
        SET role = $2, updated_at = NOW()
        WHERE id = $1
        "#,
        id,
        role as UserRole
    )
		.execute(pool)
		.await?;

	Ok(())
}
