use axum::{
    routing::post,
    Router,
};
use utoipa::OpenApi;

pub mod login;
use login::__path_login;
use login::login;

pub mod register;
use register::__path_register;
use register::register;
pub mod utils;
use crate::server::AppState;

#[derive(OpenApi)]
#[openapi(paths(register, login))]
pub struct AuthApi;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(app_state)
}
