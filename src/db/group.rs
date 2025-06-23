use crate::models::group::Group;

use super::Database;
use sqlx::Row;

impl Database {
    pub async fn create_group(&self, group: &Group) -> Result<u32, sqlx::Error> {
        let query = "INSERT INTO groups (name, description, owner_id, group_start_date, group_end_date, location) VALUES (?, ?, ?, ?, ?, ?) RETURNING id";
        let id = sqlx::query(query)
            .bind(group.name.clone())
            .bind(group.description.clone())
            .bind(group.owner_id)
            .bind(group.group_start_date)
            .bind(group.group_end_date)
            .bind(group.location.clone())
            .fetch_one(&self.pool)
            .await?;
        let id = id.get("id");
        Ok(id)
    }

    pub async fn get_group(&self, group_id: u32) -> Result<Group, sqlx::Error> {
        let query = "SELECT * FROM groups WHERE id = ?";
        let row = sqlx::query(query)
            .bind(group_id)
            .fetch_one(&self.pool)
            .await?;

        let group = Group {
            id: Some(row.get("id")),
            name: row.get("name"),
            owner_id: row.get("owner_id"),
            group_start_date: row.get("group_start_date"),
            group_end_date: row.get("group_end_date"),
            description: row.get("description"),
            location: row.get("location"),
        };
        Ok(group)
    }

    pub async fn get_groups_by_owner_id(&self, owner_id: u32) -> Result<Vec<Group>, sqlx::Error> {
        let query = "SELECT * FROM groups WHERE owner_id = ?";
        let rows = sqlx::query(query)
            .bind(owner_id)
            .fetch_all(&self.pool)
            .await?;
        let groups = rows
            .into_iter()
            .map(|row| Group {
                id: Some(row.get("id")),
                name: row.get("name"),
                owner_id: row.get("owner_id"),
                group_start_date: row.get("group_start_date"),
                group_end_date: row.get("group_end_date"),
                description: row.get("description"),
                location: row.get("location"),
            })
            .collect();

        Ok(groups)
    }

    pub async fn add_user_to_group(&self, group_id: u32, user_id: u32) -> Result<(), sqlx::Error> {
        let query = "INSERT INTO group_members (group_id, user_id) VALUES (?, ?)";
        sqlx::query(query)
            .bind(group_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_group_members(&self, group_id: u32) -> Result<Vec<u32>, sqlx::Error> {
        let query = "SELECT user_id FROM group_members WHERE group_id = ?";
        let rows = sqlx::query(query)
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;
        let members = rows.into_iter().map(|row| row.get("user_id")).collect();
        Ok(members)
    }
    pub async fn get_user_groups(&self, user_id: u32) -> Result<Vec<u32>, sqlx::Error> {
        let query = "SELECT group_id FROM group_members WHERE user_id = ?";
        let rows = sqlx::query(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        let groups = rows.into_iter().map(|row| row.get("group_id")).collect();
        Ok(groups)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{db::tests::IN_MEMORY_DB, models::user::User};

    use super::*;

    async fn setup_test_env() -> (Database, u32) {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
        let user = User::new("Test User", "test@example.com", "password");
        let user_id = db.create_user(&user, "token").await.unwrap();
        (db, user_id)
    }

    #[tokio::test]
    async fn test_create_group() {
        let (db, user_id) = setup_test_env().await;

        let time = Utc::now();
        let group = Group {
            id: None,
            name: "Test Group".to_string(),
            owner_id: user_id,
            group_start_date: time,
            group_end_date: time,
            description: "Test Description".to_string(),
            location: "Test Location".to_string(),
        };
        db.create_group(&group).await.unwrap();
        db.create_group(&group).await.unwrap();
        db.create_group(&group).await.unwrap();

        let group = db.get_group(1).await.unwrap();
        let groups = db.get_groups_by_owner_id(1).await.unwrap();

        assert_eq!(groups.len(), 3);
        assert_eq!(group.name, "Test Group");
        assert_eq!(group.owner_id, 1);
        assert_eq!(group.group_start_date, time);
        assert_eq!(group.group_end_date, time);
        assert_eq!(group.description, "Test Description");
        assert_eq!(group.location, "Test Location");
    }

    #[tokio::test]
    async fn test_no_existing_group() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
        let group = Group {
            id: None,
            name: "Test Group".to_string(),
            owner_id: 1,
            group_start_date: Utc::now(),
            group_end_date: Utc::now(),
            description: "Test Description".to_string(),
            location: "Test Location".to_string(),
        };
        assert!(db.create_group(&group).await.is_err());
    }

    #[tokio::test]
    async fn test_add_user_to_group() {
        let (db, user_id) = setup_test_env().await;

        let time = Utc::now();
        let group = Group {
            id: None,
            name: "Test Group".to_string(),
            owner_id: user_id,
            group_start_date: time,
            group_end_date: time,
            description: "Test Description".to_string(),
            location: "Test Location".to_string(),
        };
        let group_id = db.create_group(&group).await.unwrap();

        db.add_user_to_group(group_id, user_id).await.unwrap();
        assert!(
            db.add_user_to_group(group_id, user_id).await.is_err(),
            "Adding user to group should fail if user is already in group"
        );

        let members = db.get_group_members(group_id).await.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0], user_id);

        let group_id = db.create_group(&group).await.unwrap();
        db.add_user_to_group(group_id, user_id).await.unwrap();

        let groups = db.get_user_groups(user_id).await.unwrap();
        assert_eq!(groups.len(), 2);
        assert!(groups.contains(&group_id));
    }

    #[tokio::test]
    async fn test_get_nonexistent_group() {
        let (db, _) = setup_test_env().await;

        // Try to get non-existent group
        let result = db.get_group(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_user_to_nonexistent_group() {
        let (db, user_id) = setup_test_env().await;

        // Try to add user to non-existent group
        let result = db.add_user_to_group(999, user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_members_nonexistent_group() {
        let (db, _) = setup_test_env().await;

        // Try to get members of non-existent group
        let result = db.get_group_members(999).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_multiple_users_in_group() {
        let (db, owner_id) = setup_test_env().await;

        // Create additional users
        let user1 = User::new("User 1", "user1@example.com", "pass1");
        let user2 = User::new("User 2", "user2@example.com", "pass2");
        let user1_id = db.create_user(&user1, "token1").await.unwrap();
        let user2_id = db.create_user(&user2, "token2").await.unwrap();

        // Create group
        let group = Group {
            id: None,
            name: "Multi-User Group".to_string(),
            owner_id,
            group_start_date: Utc::now(),
            group_end_date: Utc::now(),
            description: "Test Group".to_string(),
            location: "Test Location".to_string(),
        };
        let group_id = db.create_group(&group).await.unwrap();

        // Add multiple users
        db.add_user_to_group(group_id, user1_id).await.unwrap();
        db.add_user_to_group(group_id, user2_id).await.unwrap();

        // Verify group members
        let members = db.get_group_members(group_id).await.unwrap();
        assert_eq!(members.len(), 2);
        assert!(members.contains(&user1_id));
        assert!(members.contains(&user2_id));

        // Verify each user's groups
        let user1_groups = db.get_user_groups(user1_id).await.unwrap();
        let user2_groups = db.get_user_groups(user2_id).await.unwrap();
        assert_eq!(user1_groups.len(), 1);
        assert_eq!(user2_groups.len(), 1);
        assert_eq!(user1_groups[0], group_id);
        assert_eq!(user2_groups[0], group_id);
    }
}
