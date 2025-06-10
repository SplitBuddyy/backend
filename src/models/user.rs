use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
impl User {
    pub fn new(id: i32, name: &str, email: &str, password: &str) -> User {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User: {}\nEmail: {}", self.name, self.email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let user = User::new(1, "Alice", "alice@example.com", "password123");
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.password, "password123");
    }

    #[test]
    fn test_user_display() {
        let user = User::new(2, "Bob", "bob@example.com", "secret");
        let display = format!("{}", user);
        assert!(display.contains("User: Bob"));
        assert!(display.contains("Email: bob@example.com"));
    }
}
