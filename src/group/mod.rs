use crate::server::AppState;
use axum::{routing::post, Router};

mod add_expense;
use add_expense::__path_add_expense;
use add_expense::add_expense;

mod join_group;
use join_group::__path_join_group;
use join_group::join_group;

mod calculate_expense;
use calculate_expense::__path_calculate_expense;
use calculate_expense::calculate_expense;

mod create_group;
use create_group::__path_create_group;
use create_group::create_group;

mod get_groups;
use get_groups::__path_get_user_groups;
use get_groups::get_user_groups;

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(paths(
    get_user_groups,
    create_group,
    join_group,
    calculate_expense,
    add_expense
))]
pub struct GroupApi;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/create_group", post(create_group))
        .route("/get_user_groups", post(get_user_groups))
        .route("/join_group", post(join_group))
        .route("/add_expense", post(add_expense))
        .route("/calculate", post(calculate_expense))
        .with_state(app_state)
}
