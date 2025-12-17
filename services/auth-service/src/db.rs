use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use anyhow::Result;
use crate::logic::models::{DiscordUser, UserRole};

pub async fn connect(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn save_user_and_tokens(
    db_pool: &sqlx::PgPool,
    user: &DiscordUser,
    role: UserRole,
    refresh_token: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, discriminator, avatar, role)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
            SET username = EXCLUDED.username,
                discriminator = EXCLUDED.discriminator,
                avatar = EXCLUDED.avatar,
                role = EXCLUDED.role
        "#,
        user.id,
        user.username,
        user.discriminator,
        user.avatar,
        role.clone() as UserRole,
    )
        .execute(db_pool)
        .await?;

    if let Some(rt) = refresh_token {
        sqlx::query!(
            r#"
            INSERT INTO oauth_tokens (user_id, refresh_token)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO UPDATE
                SET refresh_token = EXCLUDED.refresh_token
            "#,
            user.id,
            rt
        )
            .execute(db_pool)
            .await?;
    }

    Ok(())
}
