pub mod group;
pub mod models;
pub mod server;
pub mod user;

#[tokio::main]
async fn main() {
    server::start().await;
}
