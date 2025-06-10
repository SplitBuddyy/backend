use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    pub id: i32,
    pub payer: User,
    pub receiver: User,
    pub amount: f64,
    pub date: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Expense {
    pub id: i32,
    pub description: Option<String>,
    pub amount: f64,
    pub payer: User,
    pub participants: Vec<User>,
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::User;

    #[test]
    fn test_transaction_creation() {
        let payer = User::new(1, "Alice", "alice@example.com", "pass");
        let receiver = User::new(2, "Bob", "bob@example.com", "pass");
        let tx = Transaction {
            id: 1,
            payer: payer.clone(),
            receiver: receiver.clone(),
            amount: 50.0,
            date: "2024-01-01".to_string(),
        };
        assert_eq!(tx.id, 1);
        assert_eq!(tx.payer.name, "Alice");
        assert_eq!(tx.receiver.name, "Bob");
        assert_eq!(tx.amount, 50.0);
        assert_eq!(tx.date, "2024-01-01");
    }

    #[test]
    fn test_expense_creation() {
        let payer = User::new(3, "Carol", "carol@example.com", "pass");
        let participants = vec![payer.clone()];
        let expense = Expense {
            id: 2,
            description: Some("Lunch".to_string()),
            amount: 20.0,
            payer: payer.clone(),
            participants: participants.clone(),
            date: "2024-01-02".to_string(),
        };
        assert_eq!(expense.id, 2);
        assert_eq!(expense.description.as_deref(), Some("Lunch"));
        assert_eq!(expense.amount, 20.0);
        assert_eq!(expense.payer.name, "Carol");
        assert_eq!(expense.participants.len(), 1);
        assert_eq!(expense.date, "2024-01-02");
    }
}
