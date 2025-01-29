use axum::{
    routing::{get, post},
    Router,
};

use crate::server::AppState;

mod add_user;
use add_user::create_user;

mod get_user;
use get_user::get_users;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/create_user", post(create_user))
        .route("/get_users", get(get_users))
        .with_state(app_state)
}
