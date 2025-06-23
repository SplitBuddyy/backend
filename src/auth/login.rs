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
    // Generate token the same way as in register
    let mut hasher = Sha256::new();
    hasher.update(user.email.as_bytes());
    hasher.update(user.password.as_bytes());
    let result = hasher.finalize();
    let token = general_purpose::STANDARD.encode(result);

    let user_id = app_state.db.get_user_id_by_token(&token).await;
    // Check if token exists in api_tokens
    if user_id.is_ok() {
        Response::new(token)
    } else {
        Response::new("Invalid credentials".to_string())
    }
}
