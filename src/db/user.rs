use crate::{db::Database, models::user::User};
use sqlx::Row;

impl Database {
    pub async fn create_user(&self, user: &User, token: &str) -> Result<u32, sqlx::Error> {
        let query = "INSERT INTO users (name, email, password) VALUES (?, ?, ?) RETURNING id";
        let row = sqlx::query(query)
            .bind(user.name.clone())
            .bind(user.email.clone())
            .bind(user.password.clone())
            .fetch_one(&self.pool)
            .await?;
        let id = row.get("id");
        let query = "INSERT INTO api_tokens (token, user_id) VALUES (?, ?)";
        sqlx::query(query)
            .bind(token)
            .bind(id)
            .execute(&self.pool)
            .await?;
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

    pub async fn get_user_id_by_token(&self, token: &str) -> Result<u32, sqlx::Error> {
        let query = "SELECT * FROM api_tokens WHERE token = ?";
        let row = sqlx::query(query).bind(token).fetch_one(&self.pool).await?;
        let user_id = row.get("user_id");
        Ok(user_id)
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

        let user = User::new("Test User", "test@example.com", "password");
        let id = db.create_user(&user, "token").await.unwrap();
        let token_id = db.get_user_id_by_token("token").await.unwrap();
        assert_eq!(id, token_id);
        let user = db.get_user(id).await.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password, "password");
    }

    #[tokio::test]
    async fn test_unique_user_mail() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
        let user = User::new("Test User", "test@example.com", "password");
        let user2 = User::new("Test User 2", "test@example.com", "password2");
        db.create_user(&user, "token").await.unwrap();
        assert!(
            db.create_user(&user2, "token2").await.is_err(),
            "User with the same email should not be created"
        );
    }
}
