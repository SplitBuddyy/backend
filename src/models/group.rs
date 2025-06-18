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
pub struct ExpenseAddRequest {
    pub group_id: u32,
    pub expense: Expense,
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::models::expenses::Expense;
//     use crate::models::user::User;

//     fn sample_user(id: u32, name: &str) -> User {
//         let mut user = User::new(name, &format!("{}@example.com", name), "pass");
//         user.id = Some(id);
//         user
//     }

//     #[test]
//     fn test_group_new() {
//         let group = Group::new(
//             "Trip",
//             42,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         assert_eq!(group.id, 1);
//         assert_eq!(group.name, "Trip");
//         assert_eq!(group.owner_id, 42);
//         assert!(group.expenses.is_empty());
//     }

//     #[test]
//     fn test_add_members() {
//         let mut group = Group::new(
//             2,
//             "Party",
//             7,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user = sample_user(1, "Alice");
//         group.add_members(user.id.unwrap());
//         assert_eq!(group.members_ids.len(), 2);
//         assert_eq!(group.members_ids[1], user.id.unwrap());
//     }

//     #[test]
//     fn test_add_expense() {
//         let mut group = Group::new(
//             3,
//             "Dinner",
//             8,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user = sample_user(2, "Bob");
//         let expense = Expense {
//             id: 1,
//             description: Some("Pizza".to_string()),
//             amount: 30.0,
//             payer_id: user.id.unwrap(),
//             participants_ids: vec![user.id.unwrap()],
//             date: "2024-01-01".to_string(),
//         };
//         group.add_expense(expense.clone());
//         assert_eq!(group.expenses.len(), 1);
//         assert_eq!(group.expenses[0].amount, 30.0);
//     }

//     #[test]
//     fn test_group_summary() {
//         let mut group = Group::new(
//             4,
//             "Lunch",
//             9,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user = sample_user(3, "Carol");
//         let expense = Expense {
//             id: 2,
//             description: Some("Sandwiches".to_string()),
//             amount: 20.0,
//             payer_id: user.id.unwrap(),
//             participants_ids: vec![user.id.unwrap()],
//             date: "2024-01-02".to_string(),
//         };
//         group.add_expense(expense.clone());
//         let summary = get_group_summary(group);
//         assert_eq!(summary.total_spent, 20.0);
//         assert_eq!(summary.expenses.len(), 1);
//         assert_eq!(summary.transactions.len(), 1);
//         assert_eq!(summary.transactions[0].amount, 20.0);
//     }

//     #[test]
//     fn test_group_display() {
//         let group = Group::new(
//             5,
//             "TestGroup",
//             10,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let display = format!("{}", group);
//         assert!(display.contains("Group: TestGroup"));
//     }

//     #[test]
//     fn test_group_summary_display() {
//         let group = Group::new(
//             6,
//             "SumGroup",
//             11,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let summary = GroupSummary {
//             group,
//             total_spent: 100.0,
//             expenses: vec![],
//             transactions: vec![],
//         };
//         let display = format!("{}", summary);
//         assert!(display.contains("Group: SumGroup"));
//         assert!(display.contains("Total Spent: 100"));
//     }

//     #[test]
//     fn test_add_multiple_members() {
//         let mut group = Group::new(
//             7,
//             "MultiMembers",
//             12,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user1 = sample_user(4, "Dave");
//         let user2 = sample_user(5, "Eve");
//         group.add_members(user1.id.unwrap());
//         group.add_members(user2.id.unwrap());
//         assert_eq!(group.members_ids.len(), 3);
//         assert_eq!(group.members_ids[2], user2.id.unwrap());
//     }

//     #[test]
//     fn test_add_multiple_expenses() {
//         let mut group = Group::new(
//             8,
//             "MultiExpenses",
//             13,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user = sample_user(6, "Frank");
//         let expense1 = Expense {
//             id: 3,
//             description: Some("Coffee".to_string()),
//             amount: 10.0,
//             payer_id: user.id.unwrap(),
//             participants_ids: vec![user.id.unwrap()],
//             date: "2024-01-03".to_string(),
//         };
//         let expense2 = Expense {
//             id: 4,
//             description: Some("Bagel".to_string()),
//             amount: 5.0,
//             payer_id: user.id.unwrap(),
//             participants_ids: vec![user.id.unwrap()],
//             date: "2024-01-04".to_string(),
//         };
//         group.add_expense(expense1);
//         group.add_expense(expense2);
//         assert_eq!(group.expenses.len(), 2);
//         assert_eq!(group.expenses[1].amount, 5.0);
//     }

//     #[test]
//     fn test_add_duplicate_member() {
//         let mut group = Group::new(
//             9,
//             "DupMember",
//             14,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user = sample_user(7, "Grace");
//         group.add_members(user.id.unwrap());
//         group.add_members(user.id.unwrap());
//         assert_eq!(group.members_ids.len(), 2);
//     }

//     #[test]
//     fn test_expense_with_no_participants() {
//         let mut group = Group::new(
//             10,
//             "NoParticipants",
//             15,
//             Utc::now(),
//             Utc::now(),
//             "Description".to_string(),
//             "Location".to_string(),
//         );
//         let user = sample_user(8, "Heidi");
//         let expense = Expense {
//             id: 5,
//             description: Some("Solo".to_string()),
//             amount: 50.0,
//             payer_id: user.id.unwrap(),
//             participants_ids: vec![],
//             date: "2024-01-05".to_string(),
//         };
//         group.add_expense(expense.clone());
//         let summary = get_group_summary(group);
//         assert_eq!(summary.expenses.len(), 1);
//         // No transactions should be created if no participants
//         assert_eq!(summary.transactions.len(), 0);
//     }
// }
