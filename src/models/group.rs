//use core::fmt;

use serde::{Deserialize, Serialize};

// use crate::models::{
//     expenses::{Expense, Transaction},
//     user::User,
// };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<i64>,
    pub name: String,
    pub owner: i64
}

// #[derive(Deserialize)]
// pub struct GroupRequest {
//     pub owner: u32,
//     pub group_id: u32,
// }
// #[derive(Deserialize)]
// pub struct ExpenseAddRequest{
//     pub group_info: GroupRequest,
//     pub expense: Expense
// }
// #[derive(Deserialize)]
// pub struct AddMemberRequest{
//     pub group_info: GroupRequest,
//     pub member: User
// }

// #[derive(Debug, Clone,Serialize,Deserialize)]
// pub struct GroupSummary {
//     pub group: Group,
//     pub total_spent: f64,
//     pub expenses: Vec<Expense>,
//     pub transactions: Vec<Transaction>,
// }
// impl fmt::Display for GroupSummary {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Group: {}\nTotal Spent: {}\nExpenses: {:?}\nTransactions: {:?}",
//             self.group.name, self.total_spent, self.expenses, self.transactions
//         )
//     }
// }

// impl Group {
//     pub fn new(id: u32, name: &str,owner:u32) -> Self {
//         Self {
//             id,
//             owner,
//             name: name.to_string(),
//             members:Vec::new(),
//             expenses: Vec::new(),
//         }
//     }
//     pub fn add_members(self: &mut Self,member: User){
//         self.members.push(member);
//     }
//     pub fn add_expense(self: &mut Self, expense: Expense) {
//         self.expenses.push(expense);
//     }
//     pub fn get_group_summary(self: &Self) -> GroupSummary {
//         let mut total_spent = 0.0;
//         let mut transactions: Vec<Transaction> = Vec::new();
//         for expense in &self.expenses {
//             total_spent += expense.amount;
//             let share = expense.amount / expense.participants.len() as f64;
//             for participant in &expense.participants {
//                 let transaction = Transaction {
//                     id: transactions.len() as i32 + 1,
//                     payer: expense.payer.clone(),
//                     receiver: participant.clone(),
//                     amount: share,
//                     date: expense.date.clone(),
//                 };
//                 transactions.push(transaction);
//             }
//         }
//         GroupSummary {
//             group: self.clone(),
//             total_spent,
//             expenses: self.expenses.clone(),
//             transactions,
//         }
//     }
// }
// impl fmt::Display for Group {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Group: {}\nMembers: {:?}", self.name, self.members)
//     }
// }
