use sqlx::{PgPool, Result};
use crate::logic::models::{User, UserRole};

/// Insert or update a user by internal id
pub async fn insert_user(
	pool: &PgPool,
	username: &str,
	avatar: Option<&str>,
	role: UserRole,
) -> Result<User> {
	sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, avatar, role)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            username,
            avatar,
            role AS "role: UserRole",
            created_at,
            updated_at,
            last_login,
            login_count
        "#,
        username,
        avatar,
        role as UserRole
    )
		.fetch_one(pool)
		.await
}


/// Fetch user by id
pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>> {
	sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            avatar,
            role AS "role: UserRole",
            created_at,
            updated_at,
            last_login,
            login_count
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
	id: i32,
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
