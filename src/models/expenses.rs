use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    pub id: i32,
    pub payer_id: u32,
    pub receiver_id: u32,
    pub amount: f64,
    pub date: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Expense {
    pub id: u32,
    pub description: Option<String>,
    pub amount: f64,
    pub payer_id: u32,
    pub participants_ids: Vec<u32>,
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
            payer_id: payer.id,
            receiver_id: receiver.id,
            amount: 50.0,
            date: "2024-01-01".to_string(),
        };
        assert_eq!(tx.id, 1);
        assert_eq!(tx.payer_id, 1);
        assert_eq!(tx.receiver_id, 2);
        assert_eq!(tx.amount, 50.0);
        assert_eq!(tx.date, "2024-01-01");
    }

    #[test]
    fn test_expense_creation() {
        let payer = User::new(3, "Carol", "carol@example.com", "pass");
        let participants = vec![payer.id];
        let expense = Expense {
            id: 2,
            description: Some("Lunch".to_string()),
            amount: 20.0,
            payer_id: payer.id,
            participants_ids: participants.clone(),
            date: "2024-01-02".to_string(),
        };
        assert_eq!(expense.id, 2);
        assert_eq!(expense.description.as_deref(), Some("Lunch"));
        assert_eq!(expense.amount, 20.0);
        assert_eq!(expense.payer_id, 3);
        assert_eq!(expense.participants_ids.len(), 1);
        assert_eq!(expense.participants_ids[0], 3);
        assert_eq!(expense.date, "2024-01-02");
    }
}
