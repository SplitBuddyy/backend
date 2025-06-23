use crate::{
    db::Database,
    models::expenses::{Expense, Transaction as DetailedTransaction},
};
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserBalance {
    pub user_id: u32,
    pub total_paid: f64,
    pub total_owed: f64,
    pub net_balance: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GroupSummary {
    pub group_id: u32,
    pub total_expenses: f64,
    pub transactions_needed: Vec<Transaction>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Transaction {
    pub id: Option<u32>,
    pub from_user_id: u32,
    pub to_user_id: u32,
    pub amount: f64,
}

impl Database {
    pub async fn get_group_summary(&self, group_id: u32) -> Result<GroupSummary, sqlx::Error> {
        // Get all expenses for the group
        let expenses = self.get_expenses_by_group_id(group_id).await?;

        let mut total_expenses = 0.0;

        // Calculate expenses and splits
        for expense in &expenses {
            total_expenses += expense.amount;
        }
        let rounded = (total_expenses * 100.0).round() / 100.0; // Calculate needed transactions
        let transactions_needed = self
            .calculate_optimal_transactions(expenses, group_id)
            .await;

        Ok(GroupSummary {
            group_id,
            total_expenses: rounded,
            transactions_needed,
        })
    }
    async fn calculate_optimal_transactions(
        &self,
        expenses: Vec<Expense>,
        group_id: u32,
    ) -> Vec<Transaction> {
        //TODO: remove previous transactions, and create new ones
        self.delete_transactions_by_group_id(group_id)
            .await
            .unwrap();
        let mut transactions = vec![];
        for expense in expenses {
            let payer = expense.payer_id;
            let participants = self
                .get_expense_participants(expense.id.unwrap())
                .await
                .unwrap();
            let amount = expense.amount;
            let amount_per_participant = amount / participants.len() as f64;
            for participant in participants {
                if participant == payer {
                    continue;
                }
                transactions.push(Transaction {
                    id: None,
                    from_user_id: payer,
                    to_user_id: participant,
                    amount: amount_per_participant,
                });
            }
        }
        transactions = minimize(transactions);
        for transaction in &mut transactions {
            let id = self
                .create_transaction(&DetailedTransaction {
                    payer_id: transaction.from_user_id,
                    receiver_id: transaction.to_user_id,
                    amount: transaction.amount,
                    id: None,
                    date: Utc::now().naive_utc().to_string(),
                    status: crate::models::expenses::Status::Pending,
                    group_id,
                })
                .await
                .unwrap();
            transaction.id = Some(id);
        }
        transactions
    }
}

fn minimize(transactions: Vec<Transaction>) -> Vec<Transaction> {
    let mut saldo: HashMap<u32, f64> = HashMap::new();

    for t in &transactions {
        *saldo.entry(t.from_user_id).or_insert(0.0) -= t.amount;
        *saldo.entry(t.to_user_id).or_insert(0.0) += t.amount;
    }

    // Osobno dłużnicy i wierzyciele
    let mut creditors: Vec<(u32, f64)> = saldo
        .iter()
        .filter(|(_, &v)| v > 1e-6)
        .map(|(&k, &v)| (k, v))
        .collect();

    let mut debtors: Vec<(u32, f64)> = saldo
        .iter()
        .filter(|(_, &v)| v < -1e-6)
        .map(|(&k, &v)| (k, v))
        .collect();

    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    debtors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut result = Vec::new();

    while !creditors.is_empty() && !debtors.is_empty() {
        let (creditor_id, mut credit_amount) = creditors.pop().unwrap();
        let (debtor_id, mut debt_amount) = debtors.pop().unwrap();

        debt_amount = -debt_amount;
        let amount = credit_amount.min(debt_amount);

        result.push(Transaction {
            id: None,
            from_user_id: debtor_id,
            to_user_id: creditor_id,
            amount: (amount * 100.0).round() / 100.0,
        });

        credit_amount -= amount;
        debt_amount -= amount;

        if credit_amount > 1e-6 {
            creditors.push((creditor_id, credit_amount));
            creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        }

        if debt_amount > 1e-6 {
            debtors.push((debtor_id, -debt_amount));
            debtors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let transactions = vec![
            Transaction {
                id: None,
                from_user_id: 2,
                to_user_id: 1,
                amount: 2.5,
            },
            Transaction {
                id: None,
                from_user_id: 1,
                to_user_id: 2,
                amount: 12.5,
            },
        ];

        let result = minimize(transactions);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from_user_id, 1);
        assert_eq!(result[0].to_user_id, 2);
        assert_eq!(result[0].amount, 10.0);
    }

    #[test]
    fn test_three_users() {
        let transactions = vec![
            Transaction {
                id: None,
                from_user_id: 1,
                to_user_id: 2,
                amount: 10.0,
            },
            Transaction {
                id: None,
                from_user_id: 2,
                to_user_id: 3,
                amount: 10.0,
            },
            Transaction {
                id: None,
                from_user_id: 3,
                to_user_id: 1,
                amount: 10.0,
            },
        ];

        let result = minimize(transactions);

        assert_eq!(result.len(), 0);
    }
    #[test]
    fn test_four_users() {
        let transactions = vec![
            Transaction {
                id: None,
                from_user_id: 1,
                to_user_id: 2,
                amount: 10.0,
            },
            Transaction {
                id: None,
                from_user_id: 1,
                to_user_id: 3,
                amount: 10.0,
            },
            Transaction {
                id: None,
                from_user_id: 1,
                to_user_id: 4,
                amount: 10.0,
            },
            Transaction {
                id: None,
                from_user_id: 2,
                to_user_id: 1,
                amount: 10.0,
            },
        ];

        let result = minimize(transactions);
        assert_eq!(result.len(), 2);
    }
}
