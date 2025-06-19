pub mod auth;
pub mod db;
pub mod expense;
pub mod group;
pub mod models;
pub mod server;
pub mod summary;

#[tokio::main]
async fn main() {
    server::start().await;
}
