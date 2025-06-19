use crate::{
    db::Database,
    models::expenses::Expense,
};
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
    pub user_balances: HashMap<u32, UserBalance>,
    pub transactions_needed: Vec<Transaction>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Transaction {
    pub from_user_id: u32,
    pub to_user_id: u32,
    pub amount: f64,
}

impl Database {
    pub async fn get_group_summary(&self, group_id: u32) -> Result<GroupSummary, sqlx::Error> {
        // Get all expenses for the group
        let expenses = self.get_expenses_by_group_id(group_id).await?;
        
        // Get all members of the group
        let members = self.get_group_members(group_id).await?;
        
        // Initialize user balances
        let mut user_balances: HashMap<u32, UserBalance> = members
            .iter()
            .map(|&user_id| {
                (
                    user_id,
                    UserBalance {
                        user_id,
                        total_paid: 0.0,
                        total_owed: 0.0,
                        net_balance: 0.0,
                    },
                )
            })
            .collect();

        let mut total_expenses = 0.0;

        // Calculate expenses and splits
        for expense in expenses {
            total_expenses += expense.amount;
            
            // Get participants for this expense
            let participants = self.get_expense_participants(expense.id.unwrap()).await?;
            let split_amount = expense.amount / participants.len() as f64;

            // Update payer's balance
            if let Some(payer_balance) = user_balances.get_mut(&expense.payer_id) {
                payer_balance.total_paid += expense.amount;
                if participants.contains(&expense.payer_id) {
                    payer_balance.total_owed += split_amount;
                }
            }

            // Update participants' balances
            for participant_id in participants {
                if let Some(participant_balance) = user_balances.get_mut(&participant_id) {
                    participant_balance.total_owed += split_amount;
                }
            }
        }

        // Calculate net balances
        for balance in user_balances.values_mut() {
            balance.net_balance = balance.total_paid - balance.total_owed;
        }

        // Calculate needed transactions
        let transactions_needed = calculate_optimal_transactions(&user_balances);

        Ok(GroupSummary {
            group_id,
            total_expenses,
            user_balances,
            transactions_needed,
        })
    }
}

// Helper function to calculate optimal transactions to settle debts
fn calculate_optimal_transactions(balances: &HashMap<u32, UserBalance>) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    let mut net_balances: Vec<(u32, f64)> = balances
        .values()
        .map(|b| (b.user_id, b.net_balance))
        .collect();

    // Sort by balance (negative balances first)
    net_balances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut i = 0;
    let mut j = net_balances.len() - 1;

    while i < j {
        let (debtor_id, mut debt) = net_balances[i];
        let (creditor_id, mut credit) = net_balances[j];

        if debt.abs() < 0.01 || credit < 0.01 {
            // Skip if amounts are effectively zero
            if debt.abs() < 0.01 { i += 1; }
            if credit < 0.01 { j -= 1; }
            continue;
        }

        let transfer_amount = debt.abs().min(credit);
        transactions.push(Transaction {
            from_user_id: debtor_id,
            to_user_id: creditor_id,
            amount: transfer_amount,
        });

        // Update balances
        net_balances[i].1 += transfer_amount;
        net_balances[j].1 -= transfer_amount;

        // Move indices if balance is settled
        if (net_balances[i].1.abs() < 0.01) { i += 1; }
        if (net_balances[j].1 < 0.01) { j -= 1; }
    }

    transactions
}