use axum::{extract::State, Json};
use sqlx::query_as;
use crate::{models::user::User, server::AppState};

pub async fn get_users(State(app_state): State<AppState>) -> Json<Vec<User>> {
    let users = query_as!(User, "SELECT * FROM users")
        .fetch_all(&app_state.db)
        .await
        .expect("Failed to fetch users");

    Json(users)
}
