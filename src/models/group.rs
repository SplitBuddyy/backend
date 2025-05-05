use core::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{
    expenses::{Expense, Transaction},
    user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(skip_deserializing)]
    pub id: u32,
    pub name: String,
    pub owner: u32,
    #[serde(skip_deserializing)]
    pub members: Vec<User>,
    #[serde(skip_deserializing)]
    pub expenses: Vec<Expense>,
}
#[derive(Deserialize)]
pub struct GroupRequest {
    pub owner: u32,
    pub group_id: u32,
}
#[derive(Deserialize)]
pub struct ExpenseAddRequest {
    pub group_info: GroupRequest,
    pub expense: Expense,
}
#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub group_info: GroupRequest,
    pub member: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn new(id: u32, name: &str, owner: u32) -> Self {
        Self {
            id,
            owner,
            name: name.to_string(),
            members: Vec::new(),
            expenses: Vec::new(),
        }
    }
    pub fn add_members(self: &mut Self, member: User) {
        self.members.push(member);
    }
    pub fn add_expense(self: &mut Self, expense: Expense) {
        self.expenses.push(expense);
    }
    pub fn get_group_summary(self: &Self) -> GroupSummary {
        let mut total_spent = 0.0;
        let mut transactions: Vec<Transaction> = Vec::new();
        for expense in &self.expenses {
            total_spent += expense.amount;
            let share = expense.amount / expense.participants.len() as f64;
            for participant in &expense.participants {
                let transaction = Transaction {
                    id: transactions.len() as i32 + 1,
                    payer: expense.payer.clone(),
                    receiver: participant.clone(),
                    amount: share,
                    date: expense.date.clone(),
                };
                transactions.push(transaction);
            }
        }
        GroupSummary {
            group: self.clone(),
            total_spent,
            expenses: self.expenses.clone(),
            transactions,
        }
    }
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group: {}\nMembers: {:?}", self.name, self.members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::expenses::{Expense, Transaction};
    use crate::models::user::User;

    fn sample_user(id: i32, name: &str) -> User {
        User::new(id, name, &format!("{}@example.com", name), "pass")
    }

    #[test]
    fn test_group_new() {
        let group = Group::new(1, "Trip", 42);
        assert_eq!(group.id, 1);
        assert_eq!(group.name, "Trip");
        assert_eq!(group.owner, 42);
        assert!(group.members.is_empty());
        assert!(group.expenses.is_empty());
    }

    #[test]
    fn test_add_members() {
        let mut group = Group::new(2, "Party", 7);
        let user = sample_user(1, "Alice");
        group.add_members(user.clone());
        assert_eq!(group.members.len(), 1);
        assert_eq!(group.members[0].name, "Alice");
    }

    #[test]
    fn test_add_expense() {
        let mut group = Group::new(3, "Dinner", 8);
        let user = sample_user(2, "Bob");
        let expense = Expense {
            id: 1,
            description: Some("Pizza".to_string()),
            amount: 30.0,
            payer: user.clone(),
            participants: vec![user.clone()],
            date: "2024-01-01".to_string(),
        };
        group.add_expense(expense.clone());
        assert_eq!(group.expenses.len(), 1);
        assert_eq!(group.expenses[0].amount, 30.0);
    }

    #[test]
    fn test_group_summary() {
        let mut group = Group::new(4, "Lunch", 9);
        let user = sample_user(3, "Carol");
        let expense = Expense {
            id: 2,
            description: Some("Sandwiches".to_string()),
            amount: 20.0,
            payer: user.clone(),
            participants: vec![user.clone()],
            date: "2024-01-02".to_string(),
        };
        group.add_expense(expense.clone());
        let summary = group.get_group_summary();
        assert_eq!(summary.total_spent, 20.0);
        assert_eq!(summary.expenses.len(), 1);
        assert_eq!(summary.transactions.len(), 1);
        assert_eq!(summary.transactions[0].amount, 20.0);
    }

    #[test]
    fn test_group_display() {
        let group = Group::new(5, "TestGroup", 10);
        let display = format!("{}", group);
        assert!(display.contains("Group: TestGroup"));
    }

    #[test]
    fn test_group_summary_display() {
        let group = Group::new(6, "SumGroup", 11);
        let summary = GroupSummary {
            group,
            total_spent: 100.0,
            expenses: vec![],
            transactions: vec![],
        };
        let display = format!("{}", summary);
        assert!(display.contains("Group: SumGroup"));
        assert!(display.contains("Total Spent: 100"));
    }

    #[test]
    fn test_add_multiple_members() {
        let mut group = Group::new(7, "MultiMembers", 12);
        let user1 = sample_user(4, "Dave");
        let user2 = sample_user(5, "Eve");
        group.add_members(user1.clone());
        group.add_members(user2.clone());
        assert_eq!(group.members.len(), 2);
        assert_eq!(group.members[1].name, "Eve");
    }

    #[test]
    fn test_add_multiple_expenses() {
        let mut group = Group::new(8, "MultiExpenses", 13);
        let user = sample_user(6, "Frank");
        let expense1 = Expense {
            id: 3,
            description: Some("Coffee".to_string()),
            amount: 10.0,
            payer: user.clone(),
            participants: vec![user.clone()],
            date: "2024-01-03".to_string(),
        };
        let expense2 = Expense {
            id: 4,
            description: Some("Bagel".to_string()),
            amount: 5.0,
            payer: user.clone(),
            participants: vec![user.clone()],
            date: "2024-01-04".to_string(),
        };
        group.add_expense(expense1);
        group.add_expense(expense2);
        assert_eq!(group.expenses.len(), 2);
        assert_eq!(group.expenses[1].amount, 5.0);
    }

    #[test]
    fn test_add_duplicate_member() {
        let mut group = Group::new(9, "DupMember", 14);
        let user = sample_user(7, "Grace");
        group.add_members(user.clone());
        group.add_members(user.clone());
        assert_eq!(group.members.len(), 2); // No deduplication in add_members
    }

    #[test]
    fn test_expense_with_no_participants() {
        let mut group = Group::new(10, "NoParticipants", 15);
        let user = sample_user(8, "Heidi");
        let expense = Expense {
            id: 5,
            description: Some("Solo".to_string()),
            amount: 50.0,
            payer: user.clone(),
            participants: vec![],
            date: "2024-01-05".to_string(),
        };
        group.add_expense(expense.clone());
        let summary = group.get_group_summary();
        assert_eq!(summary.expenses.len(), 1);
        // No transactions should be created if no participants
        assert_eq!(summary.transactions.len(), 0);
    }
}
