use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Expense {
    #[serde(skip_deserializing)]
    pub id: Option<u32>,
    pub description: String,
    pub amount: f64,
    pub payer_id: u32,
    pub group_id: u32,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    pub id: Option<u32>,
    pub payer_id: u32,
    pub receiver_id: u32,
    pub amount: f64,
    pub date: String,
    pub status: Status,
    pub group_id: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum Status {
    Pending,
    Completed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "pending"),
            Status::Completed => write!(f, "completed"),
        }
    }
}

impl Status {
    pub fn from_string(status: String) -> Self {
        match status.as_str() {
            "pending" => Status::Pending,
            "completed" => Status::Completed,
            _ => panic!("Invalid status"),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct ExpenseAddRequest {
    pub expense: Expense,
    pub participants_ids: Vec<u32>,
}
#[derive(Deserialize, ToSchema)]
pub struct GetExpensesByGroupIdRequest {
    pub group_id: u32,
}

#[derive(Deserialize, ToSchema)]
pub struct GetExpensesByUserIdRequest {
    pub user_id: u32,
}
