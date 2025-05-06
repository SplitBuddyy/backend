use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub owner: u32,
    pub members: Vec<User>,
    pub expenses: Vec<Expense>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: i32,
    pub description: Option<String>,
    pub amount: f64,
    pub payer: User,
    pub participants: Vec<User>,
    pub date: String,
}

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

    pub async fn create_group(&self, name: &str, owner: u32) -> reqwest::Result<Group> {
        let resp = self
            .client
            .post(&format!("{}/groups", self.base_url))
            .json(&serde_json::json!({"name": name, "owner": owner}))
            .send()
            .await?;
        resp.json::<Group>().await
    }

    pub async fn get_group(&self, group_id: u32) -> reqwest::Result<Group> {
        let resp = self
            .client
            .get(&format!("{}/groups/{}", self.base_url, group_id))
            .send()
            .await?;
        resp.json::<Group>().await
    }
}
