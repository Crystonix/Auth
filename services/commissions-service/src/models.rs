use oso::{PolarClass};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, PolarClass)]
pub struct Commission {
    #[polar(attribute)]
    pub id: i32,
    #[polar(attribute)]
    pub user_id: String,
    #[polar(attribute)]
    pub title: String,
    #[polar(attribute)]
    pub description: String,
    #[polar(attribute)]
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PolarClass)]
pub struct SessionUser {
    #[polar(attribute)]
    pub id: String,
    #[polar(attribute)]
    pub role: String, // "user" or "admin"
}