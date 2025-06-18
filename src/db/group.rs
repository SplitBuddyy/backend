use super::Database;
use chrono::{DateTime, Utc};
use sqlx::Row;

pub struct Group {
    pub id: Option<u32>,
    pub name: String,
    pub owner_id: u32,
    pub group_start_date: DateTime<Utc>,
    pub group_end_date: DateTime<Utc>,
    pub description: String,
    pub location: String,
}

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
}

#[cfg(test)]
mod tests {
    use crate::{db::tests::IN_MEMORY_DB, models::user::User};

    use super::*;

    #[tokio::test]
    async fn test_create_group() {
        let db = Database::new(IN_MEMORY_DB).await.unwrap();
        db.init().await.unwrap();
        let user = User {
            id: 12312, //Irrelevant as its not used in user creation, its db generated
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        db.create_user(&user).await.unwrap();

        let time = Utc::now();
        let group = Group {
            id: None,
            name: "Test Group".to_string(),
            owner_id: 1,
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
}
