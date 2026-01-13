use crate::models::Commission;
use sqlx::PgPool;

pub async fn get_user_commissions(db: &PgPool, user_id: &str) -> anyhow::Result<Vec<Commission>> {
	let rows = sqlx::query_as!(
        Commission,
        r#"SELECT id, user_id, title, description, status
           FROM commissions
           WHERE user_id = $1"#,
        user_id
    )
		.fetch_all(db)
		.await?;

	Ok(rows)
}

pub async fn create_commission(db: &PgPool, commission: &Commission) -> anyhow::Result<()> {
	sqlx::query!(
        "INSERT INTO commissions (user_id, title, description, status) VALUES ($1, $2, $3, $4)",
        commission.user_id,
        commission.title,
        commission.description,
        commission.status
    )
		.execute(db)
		.await?;
	Ok(())
}
