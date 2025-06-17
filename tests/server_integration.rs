mod sdk;
use axum::serve;
use portpicker::pick_unused_port;
use sdk::Sdk;
use tokio::net::TcpListener;
use trip_split::server;

async fn spawn_server_on_random_port() -> u16 {
    let port = pick_unused_port().expect("No ports free");
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("bind");
    let app = server::app();
    tokio::spawn(async move {
        serve(listener, app).await.unwrap();
    });
    // Optionally, poll until server is ready
    port
}

#[tokio::test]
async fn test_create_and_get_group() {
    let port = spawn_server_on_random_port().await;
    let base_url = format!("http://127.0.0.1:{}", port);
    let sdk = Sdk::new(&base_url);
    // Wait for server to be ready
    let mut tries = 0;
    loop {
        if let Ok(resp) = reqwest::get(format!("{}/", base_url)).await {
            if resp.status().is_success() {
                break;
            }
        }
        tries += 1;
        if tries > 20 {
            panic!("Server did not start in time");
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    // Create a user
    let api_token = sdk
        .create_user("IntegrationTestUser", "integration@test.com", "password123")
        .await
        .expect("create user");
    assert!(api_token.contains("p4IcTeRlfeOLA4rqeM8nWSWI0xfJpgtkwIJetmcUC/k="));

    // Create a group
    let resp = sdk
        .create_group("IntegrationTestGroup", 0, &api_token)
        .await
        .expect("create group");
    println!("Group created: {}", resp);

    assert!(resp.contains("Group created succesfully"));
    // Fetch all groups for owner 42
    let groups = sdk.get_groups(0, &api_token).await.expect("get groups");
    let group = groups
        .iter()
        .find(|g| g.name == "IntegrationTestGroup")
        .expect("group found");
    assert_eq!(group.name, "IntegrationTestGroup");
    assert_eq!(group.owner_id, 0);
}
