use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub payer: User,
    pub receiver: User,
    pub amount: f64,
    pub date: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub description: Option<String>,
    pub amount: f64,
    pub payer: User,
    pub participants: Vec<User>,
    pub date: String,
}
