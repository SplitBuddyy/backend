use crate::{db::Database, models::expenses::Expense};
use sqlx::Row;

impl Database {
    pub async fn create_expense(&self, expense: &Expense) -> Result<u32, sqlx::Error> {
        let query = "INSERT INTO expenses (description, amount, payer_id, group_id, date) VALUES (?, ?, ?, ?, ?) RETURNING id";
        let row = sqlx::query(query)
            .bind(expense.description.clone())
            .bind(expense.amount)
            .bind(expense.payer_id)
            .bind(expense.group_id)
            .bind(expense.date.clone())
            .fetch_one(&self.pool)
            .await?;
        let id = row.get("id");
        Ok(id)
    }

    pub async fn get_expenses_by_group_id(
        &self,
        group_id: u32,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let query = "SELECT * FROM expenses WHERE group_id = ?";
        let rows = sqlx::query(query)
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;
        let expenses = rows
            .into_iter()
            .map(|row| Expense {
                id: Some(row.get("id")),
                description: row.get("description"),
                amount: row.get("amount"),
                payer_id: row.get("payer_id"),
                group_id: row.get("group_id"),
                date: row.get("date"),
            })
            .collect();
        Ok(expenses)
    }

    pub async fn get_expenses_by_payer_id(
        &self,
        payer_id: u32,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let query = "SELECT * FROM expenses WHERE payer_id = ?";
        let rows = sqlx::query(query)
            .bind(payer_id)
            .fetch_all(&self.pool)
            .await?;
        let expenses = rows
            .into_iter()
            .map(|row| Expense {
                id: Some(row.get("id")),
                description: row.get("description"),
                amount: row.get("amount"),
                payer_id: row.get("payer_id"),
                group_id: row.get("group_id"),
                date: row.get("date"),
            })
            .collect();
        Ok(expenses)
    }

    pub async fn add_participants_to_expense(
        &self,
        expense_id: u32,
        users_ids: Vec<u32>,
    ) -> Result<(), sqlx::Error> {
        for user_id in users_ids {
            let query = "INSERT INTO expense_participants (expense_id, user_id) VALUES (?, ?)";
            sqlx::query(query)
                .bind(expense_id)
                .bind(user_id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    pub async fn get_expense_participants(&self, expense_id: u32) -> Result<Vec<u32>, sqlx::Error> {
        let query = "SELECT user_id FROM expense_participants WHERE expense_id = ?";
        let rows = sqlx::query(query)
            .bind(expense_id)
            .fetch_all(&self.pool)
            .await?;
        let participants = rows.into_iter().map(|row| row.get("user_id")).collect();
        Ok(participants)
    }

    pub async fn get_all_user_expenses(&self, user_id: u32) -> Result<Vec<u32>, sqlx::Error> {
        let query = "SELECT expense_id FROM expense_participants WHERE user_id = ?";
        let rows = sqlx::query(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        let expenses_id = rows.into_iter().map(|row| row.get("expense_id")).collect();
        Ok(expenses_id)
    }

    pub async fn get_expense_by_id(&self, expense_id: u32) -> Result<Expense, sqlx::Error> {
        let query = "SELECT * FROM expenses WHERE id = ?";
        let row = sqlx::query(query)
            .bind(expense_id)
            .fetch_one(&self.pool)
            .await?;
        let expense = Expense {
            id: Some(row.get("id")),
            description: row.get("description"),
            amount: row.get("amount"),
            payer_id: row.get("payer_id"),
            group_id: row.get("group_id"),
            date: row.get("date"),
        };
        Ok(expense)
    }

    pub async fn get_expenses_by_ids(
        &self,
        expenses_ids: Vec<u32>,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let mut expenses = Vec::new();
        let query = "SELECT * FROM expenses WHERE id = ?";
        for expense_id in expenses_ids {
            let row = sqlx::query(query)
                .bind(expense_id)
                .fetch_one(&self.pool)
                .await?;
            let expense = Expense {
                id: Some(row.get("id")),
                description: row.get("description"),
                amount: row.get("amount"),
                payer_id: row.get("payer_id"),
                group_id: row.get("group_id"),
                date: row.get("date"),
            };
            expenses.push(expense);
        }
        Ok(expenses)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::tests::IN_MEMORY_DB,
        models::{group::Group, user::User},
    };
    use chrono::Utc;

    use super::*;

    async fn setup_test_env() -> (Database, u32, u32) {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();

        // Create test user
        let user = User::new("Test User", "test@example.com", "password");
        let user_id = db.create_user(&user, "token").await.unwrap();

        // Create test group
        let group = Group {
            id: None,
            name: "Test Group".to_string(),
            owner_id: user_id,
            group_start_date: Utc::now(),
            group_end_date: Utc::now(),
            description: "Test Description".to_string(),
            location: "Test Location".to_string(),
        };
        let group_id = db.create_group(&group).await.unwrap();

        (db, user_id, group_id)
    }

    #[tokio::test]
    async fn test_create_and_get_expense() {
        let (db, user_id, group_id) = setup_test_env().await;
        let time = Utc::now().to_string();

        let expense = Expense {
            id: None,
            description: "Test Expense".to_string(),
            amount: 100.0,
            payer_id: user_id,
            group_id,
            date: time.clone(),
        };

        // Test create expense
        let expense_id = db.create_expense(&expense).await.unwrap();
        assert!(expense_id > 0);

        // Test get expense by id
        let fetched_expense = db.get_expense_by_id(expense_id).await.unwrap();
        assert_eq!(fetched_expense.description, "Test Expense");
        assert_eq!(fetched_expense.amount, 100.0);
        assert_eq!(fetched_expense.payer_id, user_id);
        assert_eq!(fetched_expense.group_id, group_id);
        assert_eq!(fetched_expense.date, time);
    }

    #[tokio::test]
    async fn test_expense_participants() {
        let (db, user_id, group_id) = setup_test_env().await;

        // Create additional participants
        let participant1 = User::new("Participant 1", "p1@example.com", "pass1");
        let participant2 = User::new("Participant 2", "p2@example.com", "pass2");
        let p1_id = db.create_user(&participant1, "token1").await.unwrap();
        let p2_id = db.create_user(&participant2, "token2").await.unwrap();

        // Create test expense
        let expense = Expense {
            id: None,
            description: "Group Dinner".to_string(),
            amount: 150.0,
            payer_id: user_id,
            group_id,
            date: Utc::now().to_string(),
        };
        let expense_id = db.create_expense(&expense).await.unwrap();

        // Add participants to expense
        db.add_participants_to_expense(expense_id, vec![p1_id, p2_id])
            .await
            .unwrap();

        // Test get expense participants
        let participants = db.get_expense_participants(expense_id).await.unwrap();
        assert_eq!(participants.len(), 2);
        assert!(participants.contains(&p1_id));
        assert!(participants.contains(&p2_id));

        // Test get expenses by participant
        let p1_expenses = db.get_all_user_expenses(p1_id).await.unwrap();
        assert_eq!(p1_expenses.len(), 1);
        assert_eq!(p1_expenses[0], expense_id);
    }

    #[tokio::test]
    async fn test_get_expenses_by_group() {
        let (db, user_id, group_id) = setup_test_env().await;

        // Create multiple expenses in the same group
        let expense1 = Expense {
            id: None,
            description: "Expense 1".to_string(),
            amount: 50.0,
            payer_id: user_id,
            group_id,
            date: Utc::now().to_string(),
        };
        let expense2 = Expense {
            id: None,
            description: "Expense 2".to_string(),
            amount: 75.0,
            payer_id: user_id,
            group_id,
            date: Utc::now().to_string(),
        };

        db.create_expense(&expense1).await.unwrap();
        db.create_expense(&expense2).await.unwrap();

        // Test get expenses by group
        let group_expenses = db.get_expenses_by_group_id(group_id).await.unwrap();
        assert_eq!(group_expenses.len(), 2);
        assert_eq!(group_expenses[0].amount, 50.0);
        assert_eq!(group_expenses[1].amount, 75.0);
    }

    #[tokio::test]
    async fn test_get_expenses_by_payer() {
        let (db, user_id, group_id) = setup_test_env().await;

        // Create another user as payer
        let other_payer = User::new("Other Payer", "other@example.com", "pass");
        let other_id = db.create_user(&other_payer, "token_other").await.unwrap();

        // Create expenses with different payers
        let expense1 = Expense {
            id: None,
            description: "User1 Expense".to_string(),
            amount: 100.0,
            payer_id: user_id,
            group_id,
            date: Utc::now().to_string(),
        };
        let expense2 = Expense {
            id: None,
            description: "User2 Expense".to_string(),
            amount: 150.0,
            payer_id: other_id,
            group_id,
            date: Utc::now().to_string(),
        };

        db.create_expense(&expense1).await.unwrap();
        db.create_expense(&expense2).await.unwrap();

        // Test get expenses by payer
        let user1_expenses = db.get_expenses_by_payer_id(user_id).await.unwrap();
        let user2_expenses = db.get_expenses_by_payer_id(other_id).await.unwrap();

        assert_eq!(user1_expenses.len(), 1);
        assert_eq!(user2_expenses.len(), 1);
        assert_eq!(user1_expenses[0].amount, 100.0);
        assert_eq!(user2_expenses[0].amount, 150.0);
    }

    #[tokio::test]
    async fn test_expense_not_found() {
        let (db, _, _) = setup_test_env().await;

        // Try to get non-existent expense
        let result = db.get_expense_by_id(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_participant_to_nonexistent_expense() {
        let (db, user_id, _) = setup_test_env().await;

        // Try to add participant to non-existent expense
        let result = db.add_participants_to_expense(999, vec![user_id]).await;
        assert!(result.is_err());
    }
}
