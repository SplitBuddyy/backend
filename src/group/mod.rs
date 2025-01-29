use axum::{routing::post, Router};

use crate::server::AppState;

mod add_expense;
mod add_to_group;
mod calculate_expense;
mod create_group;
mod get_groups;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/create_group", post(create_group::root))
        .route("/get_groups", post(get_groups::root))
        .route("/add_to_group", post(add_to_group::root))
        .route("/add_expense", post(add_expense::root))
        .route("/calculate", post(calculate_expense::root))
        .with_state(app_state)
}
