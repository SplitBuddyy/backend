#[tokio::main]
async fn main() {
    trip_split::server::start().await;
}
