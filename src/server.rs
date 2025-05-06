use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use axum::serve;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::models::group::Group;
use crate::models::user::User;
use crate::{group, user};
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub groups: Arc<Mutex<Vec<Group>>>,
}

pub fn app() -> Router {
    let app_state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
        groups: Arc::new(Mutex::new(Vec::new())),
    };
    let cors = CorsLayer::permissive();

    Router::new()
        .route("/", get(ok_handler))
        .fallback(|| async { "There is nothing here" })
        .nest("/user", user::router(app_state.clone()))
        .nest("/group", group::router(app_state.clone()))
        .layer(cors)
}

pub async fn start() {
    let app = app();
    let addr: SocketAddr = format!("{}:{}", "0.0.0.0", 3000).parse().unwrap();

    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.unwrap();
}

async fn ok_handler() -> String {
    "server is working".into()
}
