use axum::{extract::State, response::Response, Json};
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};

use crate::{models::user::User, server::AppState};

#[utoipa::path(
    post,
    path = "/login",
    request_body = User,
    responses(
        (status = 200, description = "User found"),
        (status = 404, description = "User not found")
    )
)]
pub async fn login(State(app_state): State<AppState>, Json(user): Json<User>) -> Response<String> {
    // Find user by name and password
    let users = app_state.users.lock().await;
    let found_user = users
        .iter()
        .find(|u| u.name == user.name && u.password == user.password);
    if let Some(u) = found_user {
        // Generate token the same way as in register
        let mut hasher = Sha256::new();
        hasher.update(u.name.as_bytes());
        hasher.update(u.password.as_bytes());
        let result = hasher.finalize();
        let token = general_purpose::STANDARD.encode(result);
        // Check if token exists in api_tokens
        let api_tokens = app_state.api_tokens.lock().await;
        if api_tokens.get(&token).is_some() {
            return Response::new(token);
        } else {
            return Response::new("Token not found. Please register again.".to_string());
        }
    }
    Response::new("User not found".to_string())
}
