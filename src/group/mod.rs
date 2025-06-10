use crate::server::AppState;
use axum::{routing::post, Router};

mod add_expense;
use add_expense::__path_add_expense;
use add_expense::add_expense;

mod add_to_group;
use add_to_group::__path_add_to_group;
use add_to_group::add_to_group;

mod calculate_expense;
use calculate_expense::__path_calculate_expense;
use calculate_expense::calculate_expense;

mod create_group;
use create_group::__path_create_group;
use create_group::create_group;

mod get_groups;
use get_groups::__path_get_groups;
use get_groups::get_groups;

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(paths(get_groups, create_group, add_to_group, calculate_expense, add_expense))]
pub struct GroupApi;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/create_group", post(create_group))
        .route("/get_groups", post(get_groups))
        .route("/add_to_group", post(add_to_group))
        .route("/add_expense", post(add_expense))
        .route("/calculate", post(calculate_expense))
        .with_state(app_state)
}
