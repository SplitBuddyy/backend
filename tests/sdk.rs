use chrono::Utc;
use reqwest::Client;
use serde_json;
use trip_split::models::{group::Group, user::User};

pub struct Sdk {
    client: Client,
    base_url: String,
}

impl Sdk {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> reqwest::Result<String> {
        let user = User {
            id: Some(0),
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };
        let resp = self
            .client
            .post(&format!("{}/auth/register", self.base_url))
            .json(&user)
            .send()
            .await?;
        resp.text().await
    }

    pub async fn create_group(
        &self,
        name: &str,
        owner: u32,
        api_token: &str,
    ) -> reqwest::Result<String> {
        let group = Group {
            id: 0,
            name: name.to_string(),
            owner_id: owner,
            members_ids: vec![],
            expenses: vec![],
            group_start_date: Utc::now(),
            group_end_date: Utc::now(),
            description: "Description".to_string(),
            location: "Location".to_string(),
        };
        let resp = self
            .client
            .post(&format!("{}/group/create_group", self.base_url))
            .header("todo_apikey", api_token)
            .json(&group)
            .send()
            .await?;
        resp.text().await
    }

    pub async fn get_groups(&self, owner: u32, api_token: &str) -> reqwest::Result<Vec<Group>> {
        let resp = self
            .client
            .post(&format!("{}/group/get_user_groups", self.base_url))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Vec<Group>>().await
    }
}
