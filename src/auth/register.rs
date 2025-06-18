use axum::{extract::State, response::Response, Json};
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};

use crate::{models::user::User, server::AppState};

#[utoipa::path(
    post,
    path = "/register",
    request_body = User,
    responses(
        (status = 200, description = "User created successfully", body = String)
    )
)]
pub async fn register(
    State(app_state): State<AppState>,
    Json(user): Json<User>,
) -> Response<String> {
    let mut hasher = Sha256::new();
    hasher.update(user.name.as_bytes());
    hasher.update(user.password.as_bytes());
    let result = hasher.finalize();
    let token = general_purpose::STANDARD.encode(result);

    match app_state.db.create_user(&user, &token).await {
        Ok(_) => (),
        Err(e) => return Response::new(e.to_string()),
    };

    Response::new(token)
}
