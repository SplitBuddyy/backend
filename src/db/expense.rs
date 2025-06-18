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

    pub async fn get_expenses_by_group_id(&self, group_id: u32) -> Result<Vec<Expense>, sqlx::Error> {
        let query = "SELECT * FROM expenses WHERE group_id = ?";
        let rows = sqlx::query(query)
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;
        let expenses = rows.into_iter().map(|row| Expense {
            id: Some(row.get("id")),
            description: row.get("description"),
            amount: row.get("amount"),
            payer_id: row.get("payer_id"),
            group_id: row.get("group_id"),
            date: row.get("date"),
        }).collect();
        Ok(expenses)
    }

    pub async fn get_expenses_by_payer_id(&self, payer_id: u32) -> Result<Vec<Expense>, sqlx::Error> {
        let query = "SELECT * FROM expenses WHERE payer_id = ?";
        let rows = sqlx::query(query)
            .bind(payer_id)
            .fetch_all(&self.pool)
            .await?;
        let expenses = rows.into_iter().map(|row| Expense {
            id: Some(row.get("id")),
            description: row.get("description"),
            amount: row.get("amount"),
            payer_id: row.get("payer_id"),
            group_id: row.get("group_id"),
            date: row.get("date"),
        }).collect();
        Ok(expenses)
    }

    pub async fn add_participant_to_expense(&self, expense_id: u32, user_id: u32) -> Result<(), sqlx::Error> {
        let query = "INSERT INTO expense_participants (expense_id, user_id) VALUES (?, ?)";
        sqlx::query(query)
            .bind(expense_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
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

    pub async fn get_expenses_id_by_participant_id(&self, user_id: u32) -> Result<Vec<u32>, sqlx::Error> {
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
}

#[cfg(test)]
mod tests {
    use crate::{db::tests::IN_MEMORY_DB, models::{group::Group, user::User}};

    /// Test for expense will require a group to be created first, which implies that a user is created first
    use super::*;

    #[tokio::test]
    async fn test_create_expense() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
        let user = User::new("Test User", "test@example.com", "password");
        let user_id = db.create_user(&user, "token").await.unwrap();
        let group = Group {
            id: None,
            name: "Test Group".to_string(),
            owner_id: 1,
            group_start_date: chrono::Utc::now(),
            group_end_date: chrono::Utc::now(),
            description: "Test Description".to_string(),
            location: "Test Location".to_string(),
        };
        let group_id = db.create_group(&group).await.unwrap();
        let time = chrono::Utc::now().to_string();
        let mut expense = Expense {
            id: None,
            description: "Test Expense".to_string(),
            amount: 100.0,
            payer_id: user_id,
            group_id: group_id,
            date: time.clone(),
        };
        let expense_id = db.create_expense(&expense).await.unwrap();
        expense.id = Some(expense_id);
        let expenses = db.get_expenses_by_group_id(group_id).await.unwrap();
        assert_eq!(expenses.len(), 1);
        assert_eq!(expenses[0], expense);
        let expenses = db.get_expenses_by_payer_id(user_id).await.unwrap();
        assert_eq!(expenses.len(), 1);
        assert_eq!(expenses[0], expense);
    }
    
}