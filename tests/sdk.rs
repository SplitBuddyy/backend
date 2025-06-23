use chrono::Utc;
use reqwest::Client;
use trip_split::models::{
    expenses::{Expense, Status, Transaction},
    group::Group,
    user::User,
};

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

    // User endpoints
    pub async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> reqwest::Result<String> {
        let user = User {
            id: None,
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

    // Group endpoints
    pub async fn create_group(
        &self,
        name: &str,
        description: &str,
        location: &str,
        owner: u32,
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
        api_token: &str,
    ) -> reqwest::Result<String> {
        let group = Group {
            id: None,
            name: name.to_string(),
            owner_id: owner,
            group_start_date: start_date,
            group_end_date: end_date,
            description: description.to_string(),
            location: location.to_string(),
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
            .post(&format!("{}/group/get_user_owned_groups", self.base_url))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Vec<Group>>().await
    }

    pub async fn _add_users_to_group(
        &self,
        group_id: u32,
        user_ids: Vec<u32>,
        api_token: &str,
    ) -> reqwest::Result<()> {
        let resp = self
            .client
            .post(&format!("{}/group/{}/add_users", self.base_url, group_id))
            .header("todo_apikey", api_token)
            .json(&user_ids)
            .send()
            .await?;
        resp.error_for_status()?;
        Ok(())
    }

    pub async fn _get_group_members(
        &self,
        group_id: u32,
        api_token: &str,
    ) -> reqwest::Result<Vec<u32>> {
        let resp = self
            .client
            .get(&format!("{}/group/{}/members", self.base_url, group_id))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Vec<u32>>().await
    }

    // Expense endpoints
    pub async fn _create_expense(
        &self,
        description: &str,
        amount: f64,
        payer_id: u32,
        group_id: u32,
        participant_ids: Vec<u32>,
        api_token: &str,
    ) -> reqwest::Result<String> {
        let expense = Expense {
            id: None,
            description: description.to_string(),
            amount,
            payer_id,
            group_id,
            date: Utc::now().to_string(),
        };

        let resp = self
            .client
            .post(&format!("{}/expense/create", self.base_url))
            .header("todo_apikey", api_token)
            .json(&(expense, participant_ids))
            .send()
            .await?;
        resp.text().await
    }

    pub async fn _get_group_expenses(
        &self,
        group_id: u32,
        api_token: &str,
    ) -> reqwest::Result<Vec<Expense>> {
        let resp = self
            .client
            .get(&format!("{}/expense/group/{}", self.base_url, group_id))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Vec<Expense>>().await
    }

    pub async fn _get_user_expenses(&self, api_token: &str) -> reqwest::Result<Vec<Expense>> {
        let resp = self
            .client
            .get(&format!("{}/expense/user", self.base_url))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Vec<Expense>>().await
    }

    // Transaction endpoints
    pub async fn _create_transaction(
        &self,
        payer_id: u32,
        receiver_id: u32,
        amount: f64,
        api_token: &str,
        group_id: u32,
    ) -> reqwest::Result<String> {
        let transaction = Transaction {
            id: None,
            payer_id,
            receiver_id,
            amount,
            date: Utc::now().to_string(),
            status: Status::Pending,
            group_id,
        };

        let resp = self
            .client
            .post(&format!("{}/transaction/create", self.base_url))
            .header("todo_apikey", api_token)
            .json(&transaction)
            .send()
            .await?;
        resp.text().await
    }

    pub async fn _get_transaction(
        &self,
        transaction_id: u32,
        api_token: &str,
    ) -> reqwest::Result<Transaction> {
        let resp = self
            .client
            .get(&format!("{}/transaction/{}", self.base_url, transaction_id))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Transaction>().await
    }

    pub async fn _get_payer_transactions(
        &self,
        api_token: &str,
    ) -> reqwest::Result<Vec<Transaction>> {
        let resp = self
            .client
            .get(&format!("{}/transaction/payer", self.base_url))
            .header("todo_apikey", api_token)
            .send()
            .await?;
        resp.json::<Vec<Transaction>>().await
    }
}
