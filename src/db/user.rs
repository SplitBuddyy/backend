use crate::{db::Database, models::user::User};
use sqlx::Row;

impl Database {
    pub async fn create_user(&self, user: &User) -> Result<u32, sqlx::Error> {
        let query = "INSERT INTO users (name, email, password) VALUES (?, ?, ?) RETURNING id";
        let row = sqlx::query(query)
            .bind(user.name.clone())
            .bind(user.email.clone())
            .bind(user.password.clone())
            .fetch_one(&self.pool)
            .await?;
        let id = row.get("id");
        Ok(id)
    }

    pub async fn get_user(&self, user_id: u32) -> Result<User, sqlx::Error> {
        let query = "SELECT * FROM users WHERE id = ?";
        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        let user = User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.get("password"),
        };
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::tests::IN_MEMORY_DB;

    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();

        let user = User {
            id: 0,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };
        let id = db.create_user(&user).await.unwrap();
        let user = db.get_user(id).await.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password, "password");
    }

    #[tokio::test]
    async fn test_unique_user_mail() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
        let user = User {
            id: 0,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };
        let user2 = User {
            id: 0,
            name: "Test User 2".to_string(),
            email: "test@example.com".to_string(),
            password: "password2".to_string(),
        };
        db.create_user(&user).await.unwrap();
        assert!(
            db.create_user(&user2).await.is_err(),
            "User with the same email should not be created"
        );
    }
}
