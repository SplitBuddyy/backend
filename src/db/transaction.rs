use sqlx::Row;

use crate::{
    db::Database,
    models::expenses::{Status, Transaction},
};

impl Database {
    pub async fn create_transaction(&self, transaction: &Transaction) -> Result<u32, sqlx::Error> {
        let query = "INSERT INTO transactions (payer_id, receiver_id, amount, date, status) VALUES (?, ?, ?, ?, ?) RETURNING id";
        let row = sqlx::query(query)
            .bind(transaction.payer_id)
            .bind(transaction.receiver_id)
            .bind(transaction.amount)
            .bind(transaction.date.clone())
            .bind(transaction.status.to_string())
            .fetch_one(&self.pool)
            .await?;
        let id = row.get("id");
        Ok(id)
    }

    pub async fn get_transaction(&self, id: u32) -> Result<Transaction, sqlx::Error> {
        let query = "SELECT * FROM transactions WHERE id = ?";
        let row = sqlx::query(query).bind(id).fetch_one(&self.pool).await?;
        let transaction = Transaction {
            id: Some(row.get("id")),
            payer_id: row.get("payer_id"),
            receiver_id: row.get("receiver_id"),
            amount: row.get("amount"),
            date: row.get("date"),
            status: Status::from_string(row.get("status")),
        };
        Ok(transaction)
    }
    pub async fn get_transactions_by_payer_id(
        &self,
        payer_id: u32,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let query = "SELECT * FROM transactions WHERE payer_id = ?";
        let rows = sqlx::query(query)
            .bind(payer_id)
            .fetch_all(&self.pool)
            .await?;
        let transactions = rows
            .into_iter()
            .map(|row| Transaction {
                id: Some(row.get("id")),
                payer_id: row.get("payer_id"),
                receiver_id: row.get("receiver_id"),
                amount: row.get("amount"),
                date: row.get("date"),
                status: Status::from_string(row.get("status")),
            })
            .collect();
        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::tests::IN_MEMORY_DB, models::user::User};
    use chrono::Utc;

    #[tokio::test]
    async fn test_create_and_get_transaction() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();

        // Create two users for the transaction
        let user1 = User::new("Payer User", "payer@example.com", "password");
        let user2 = User::new("Receiver User", "receiver@example.com", "password");

        let payer_id = db.create_user(&user1, "token1").await.unwrap();
        let receiver_id = db.create_user(&user2, "token2").await.unwrap();

        let time = Utc::now().to_string();
        let transaction = Transaction {
            id: None,
            payer_id,
            receiver_id,
            amount: 100.0,
            date: time.clone(),
            status: Status::Pending,
        };

        // Test creating transaction
        let transaction_id = db.create_transaction(&transaction).await.unwrap();
        assert!(transaction_id > 0);

        // Test getting transaction
        let fetched_transaction = db.get_transaction(transaction_id).await.unwrap();
        assert_eq!(fetched_transaction.payer_id, payer_id);
        assert_eq!(fetched_transaction.receiver_id, receiver_id);
        assert_eq!(fetched_transaction.amount, 100.0);
        assert_eq!(fetched_transaction.date, time);
        assert_eq!(fetched_transaction.status, Status::Pending);
    }

    #[tokio::test]
    async fn test_get_transactions_by_payer() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();

        // Create users
        let payer = User::new("Payer", "payer@example.com", "password");
        let receiver1 = User::new("Receiver1", "receiver1@example.com", "password");
        let receiver2 = User::new("Receiver2", "receiver2@example.com", "password");

        let payer_id = db.create_user(&payer, "token_payer").await.unwrap();
        let receiver1_id = db.create_user(&receiver1, "token_receiver1").await.unwrap();
        let receiver2_id = db.create_user(&receiver2, "token_receiver2").await.unwrap();

        let time = Utc::now().to_string();

        // Create multiple transactions for the same payer
        let transaction1 = Transaction {
            id: None,
            payer_id,
            receiver_id: receiver1_id,
            amount: 50.0,
            date: time.clone(),
            status: Status::Pending,
        };

        let transaction2 = Transaction {
            id: None,
            payer_id,
            receiver_id: receiver2_id,
            amount: 75.0,
            date: time.clone(),
            status: Status::Completed,
        };

        // Create transactions
        db.create_transaction(&transaction1).await.unwrap();
        db.create_transaction(&transaction2).await.unwrap();

        // Test getting transactions by payer
        let payer_transactions = db.get_transactions_by_payer_id(payer_id).await.unwrap();
        assert_eq!(payer_transactions.len(), 2);
        assert_eq!(payer_transactions[0].amount, 50.0);
        assert_eq!(payer_transactions[1].amount, 75.0);
    }

    #[tokio::test]
    async fn test_transaction_not_found() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();

        // Try to get non-existent transaction
        let result = db.get_transaction(999).await;
        assert!(result.is_err());
    }
}
