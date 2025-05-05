use std::net::SocketAddr;
//use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use axum::serve;
use tokio::net::TcpListener;
//use tokio::sync::Mutex;

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;

//use crate::models::group::Group;
//use crate::models::user::User;

use crate::{group, user};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}
pub async fn start() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    let app_state = AppState { db: pool };
    let cors = CorsLayer::permissive();
   
    let app = Router::new()
        .route("/", get(ok_handler))
        .fallback(|| async { "There is nothing here" })
        .nest("/user", user::router(app_state.clone()))
        .nest("/group", group::router(app_state.clone()))
        .layer(cors);

    let addr: SocketAddr = format!("{}:{}", "0.0.0.0", 3000).parse().unwrap();

    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.unwrap();
}

async fn ok_handler() -> String {
    "server is working".into()
}
