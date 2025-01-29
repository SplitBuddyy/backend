use axum::{extract::State, Json};

use crate::{models::user::User, server::AppState};

pub async fn get_users(State(app_state): State<AppState>) -> Json<Vec<User>> {
    let users = app_state.users.lock().await.clone();
    Json(users)
}
