use std::time::Duration;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub mod expense;
pub mod group;
pub mod user;
#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .acquire_timeout(Duration::from_secs(10))
            .max_connections(50)
            .connect(&format!("sqlite:{}", path))
            .await?;

        Ok(Self { pool })
    }
    pub async fn init(&self) -> Result<(), sqlx::Error> {
        let schema = include_str!("../db/database.sql");
        sqlx::query(schema).execute(&self.pool).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    pub const IN_MEMORY_DB: &str = ":memory:";

    use super::*;

    #[tokio::test]
    async fn test_database_new() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
    }
}
