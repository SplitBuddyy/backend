
pub mod models;
pub mod server;
pub mod user;
pub mod group;

#[tokio::main]
async fn main() {
    server::start().await;
}