use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

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

    pub async fn create_group(&self, name: &str, owner: u32) -> reqwest::Result<String> {
        let group = Group {
            id: 0,
            name: name.to_string(),
            owner,
            members: vec![],
            expenses: vec![],
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
