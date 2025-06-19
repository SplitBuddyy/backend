use axum::{routing::post, Router};
use utoipa::OpenApi;

use crate::server::AppState;

mod add_expense;
use add_expense::__path_add_expense;
use add_expense::add_expense;

mod get_expenses_by_group;
use get_expenses_by_group::__path_get_group_expenses;
use get_expenses_by_group::get_group_expenses;

mod get_all_user_expenses;
use get_all_user_expenses::__path_get_all_user_expenses;
use get_all_user_expenses::get_all_user_expenses;

#[derive(OpenApi)]
#[openapi(paths(add_expense, get_group_expenses, get_all_user_expenses))]
pub struct ExpenseApi;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/add_expense", post(add_expense))
        .route("/get_group_expenses", post(get_group_expenses))
        .route("/get_all_user_expenses", post(get_all_user_expenses))
        .with_state(app_state)
}
