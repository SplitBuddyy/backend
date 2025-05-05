use axum::{extract::State, response::IntoResponse, Json};
use sqlx::query;
use crate::{models::user::User, server::AppState};

pub async fn create_user(
    State(app_state): State<AppState>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let result = query!(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
        user.name,
        user.email,
        user.password
    )
    .execute(&app_state.db)
    .await;

    match result {
        Ok(_) => "User created successfully".into_response(),
        Err(e) => format!("Failed to create user: {}", e).into_response(),
    }
}
