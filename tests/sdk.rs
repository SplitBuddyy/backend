use chrono::Utc;
use reqwest::Client;
use serde_json;
use trip_split::models::group::Group;

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

    pub async fn create_group(&self, name: &str, owner: u32) -> reqwest::Result<String> {
        let group = Group {
            id: 0,
            name: name.to_string(),
            owner,
            members: vec![],
            expenses: vec![],
            group_start_date: Utc::now(),
            group_end_date: Utc::now(),
        };
        let resp = self
            .client
            .post(&format!("{}/group/create_group", self.base_url))
            .json(&group)
            .send()
            .await?;
        resp.text().await
    }

    pub async fn get_groups(&self, owner: u32) -> reqwest::Result<Vec<Group>> {
        let resp = self
            .client
            .post(&format!("{}/group/get_groups", self.base_url))
            .json(&serde_json::json!({"owner": owner}))
            .send()
            .await?;
        resp.json::<Vec<Group>>().await
    }
}
