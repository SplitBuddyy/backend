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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum Status {
    Pending,
    Completed,
}
impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Pending => "pending".to_string(),
            Status::Completed => "completed".to_string(),
        }
    }
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
