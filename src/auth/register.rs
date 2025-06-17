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
    if app_state
        .users
        .lock()
        .await
        .iter()
        .any(|u| u.email == user.email)
    {
        return Response::new("User already exists".to_string());
    }
    if app_state
        .users
        .lock()
        .await
        .iter()
        .any(|u| u.name == user.name)
    {
        return Response::new("Username already exists".to_string());
    }
    if user.password.len() < 8 {
        return Response::new("Password must be at least 8 characters".to_string());
    }
    let id = if app_state.users.lock().await.is_empty() {
        0
    } else {
        app_state.users.lock().await.last().unwrap().id + 1
    };

    let user = User::new(
        id,
        user.name.as_str(),
        user.email.as_str(),
        user.password.as_str(),
    );
    println!("User created succesfully: {:?}", user);
    app_state.users.lock().await.push(user.clone());
    // Generate API token as hash of name and password
    let mut hasher = Sha256::new();
    hasher.update(user.name.as_bytes());
    hasher.update(user.password.as_bytes());
    let result = hasher.finalize();
    let token = general_purpose::STANDARD.encode(result);
    // Save the token with the user id
    app_state
        .api_tokens
        .lock()
        .await
        .insert(token.clone(), user.id);
    Response::new(token)
}
