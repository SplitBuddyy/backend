use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub password: String,
}


