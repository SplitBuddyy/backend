use core::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::expenses::{Expense, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Group {
    #[serde(skip_deserializing)]
    pub id: Option<u32>,
    pub name: String,
    pub owner_id: u32,
    pub group_start_date: DateTime<Utc>,
    pub group_end_date: DateTime<Utc>,
    pub description: String,
    pub location: String,
}
#[derive(Deserialize, ToSchema)]
pub struct GroupRequest {
    pub group_id: u32,
}
#[derive(Deserialize, ToSchema)]
pub struct JoinGroupRequest {
    pub group_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GroupSummary {
    pub group: Group,
    pub total_spent: f64,
    pub expenses: Vec<Expense>,
    pub transactions: Vec<Transaction>,
}
impl fmt::Display for GroupSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Group: {}\nTotal Spent: {}\nExpenses: {:?}\nTransactions: {:?}",
            self.group.name, self.total_spent, self.expenses, self.transactions
        )
    }
}

impl Group {
    pub fn new(
        name: &str,
        owner_id: u32,
        group_start_date: DateTime<Utc>,
        group_end_date: DateTime<Utc>,
        description: String,
        location: String,
    ) -> Self {
        Self {
            id: None,
            owner_id,
            name: name.to_string(),
            group_start_date,
            group_end_date,
            description,
            location,
        }
    }
}

// pub fn get_group_summary(group: Group) -> GroupSummary {
//     let mut total_spent = 0.0;
//     let mut transactions: Vec<Transaction> = Vec::new();
//     for expense in &group.expenses {
//         total_spent += expense.amount;
//         let share = expense.amount / expense.participants_ids.len() as f64;
//         for participant in &expense.participants_ids {
//             let transaction = Transaction {
//                 id: transactions.len() as i32 + 1,
//                 payer_id: expense.payer_id,
//                 receiver_id: *participant,
//                 amount: share,
//                 date: expense.date.clone(),
//             };
//             transactions.push(transaction);
//         }
//     }
//     GroupSummary {
//         group: group.clone(),
//         total_spent,
//         expenses: group.expenses.clone(),
//         transactions,
//     }
// }

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group: {}", self.name)
    }
}
