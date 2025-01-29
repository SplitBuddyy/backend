use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
