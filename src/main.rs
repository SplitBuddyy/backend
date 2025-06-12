pub mod auth;
pub mod group;
pub mod models;
pub mod server;

#[tokio::main]
async fn main() {
    server::start().await;
}
