use chrono::Utc;
use trip_split::models::expenses::Expense;
use trip_split::models::group::{get_group_summary, Group};
use trip_split::models::user::User;

fn sample_user(id: u32, name: &str) -> User {
    User::new(id, name, &format!("{}@example.com", name), "pass")
}

#[test]
fn integration_group_flow() {
    // Create group
    let mut group = Group::new(100, "IntegrationTest", 1, Utc::now(), Utc::now());
    let alice = sample_user(1, "Alice");
    let bob = sample_user(2, "Bob");
    let carol = sample_user(3, "Carol");

    // Add members
    group.add_members(alice.id);
    group.add_members(bob.id);
    group.add_members(carol.id);
    assert_eq!(group.members_ids.len(), 3);

    // Add expenses
    let expense1 = Expense {
        id: 1,
        description: Some("Lunch".to_string()),
        amount: 60.0,
        payer_id: alice.id,
        participants_ids: vec![alice.id, bob.id, carol.id],
        date: "2024-06-01".to_string(),
    };
    let expense2 = Expense {
        id: 2,
        description: Some("Drinks".to_string()),
        amount: 30.0,
        payer_id: bob.id,
        participants_ids: vec![alice.id, bob.id],
        date: "2024-06-02".to_string(),
    };
    group.add_expense(expense1);
    group.add_expense(expense2);
    assert_eq!(group.expenses.len(), 2);

    // Get summary
    let summary = get_group_summary(group);
    assert_eq!(summary.total_spent, 90.0);
    assert_eq!(summary.expenses.len(), 2);
    // There should be 3 (lunch) + 2 (drinks) = 5 transactions
    assert_eq!(summary.transactions.len(), 5);
    // Check transaction amounts
    let lunch_share = 60.0 / 3.0;
    let drinks_share = 30.0 / 2.0;
    assert!(summary.transactions.iter().any(|t| t.amount == lunch_share));
    assert!(summary
        .transactions
        .iter()
        .any(|t| t.amount == drinks_share));
}
