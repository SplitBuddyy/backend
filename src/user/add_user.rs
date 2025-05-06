use axum::{extract::State, response::Response, Json};

use crate::{models::user::User, server::AppState};

pub async fn create_user(
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
    Response::new(format!("User created succesfully: {:?}", user))
}
