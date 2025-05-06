mod sdk;
use sdk::Sdk;

#[tokio::test]
async fn test_create_and_get_group() {
    let sdk = Sdk::new("http://localhost:3000");
    // Create a group
    let group = sdk
        .create_group("IntegrationTestGroup", 42)
        .await
        .expect("create group");
    assert_eq!(group.name, "IntegrationTestGroup");
    assert_eq!(group.owner, 42);
    // Fetch the group
    let fetched = sdk.get_group(group.id).await.expect("get group");
    assert_eq!(fetched.id, group.id);
    assert_eq!(fetched.name, group.name);
}
