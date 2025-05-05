// use axum::{extract::State, Json};
// use serde::{Deserialize, Serialize};

// use crate::{models::group::Group, server::AppState};

// #[derive(Serialize,Deserialize)]
// pub struct GetGroupRequest{
//     pub owner:u32
// } 

// pub async fn root(State(app_state): State<AppState>,Json(payload):Json<GetGroupRequest>) -> Json<Vec<Group>> {
//     let groups = app_state.groups.lock().await.clone();
//     let owner_groups: Vec<Group> = groups.iter().filter(|g|g.owner==payload.owner).cloned().collect();
//     Json(owner_groups)
// }

use axum::{extract::State, Json};
use sqlx::query_as;
use crate::{models::group::Group, server::AppState};

pub async fn get_groups(State(app_state): State<AppState>) -> Json<Vec<Group>> {
    let users = query_as!(Group, "SELECT * FROM groups")
        .fetch_all(&app_state.db)
        .await
        .expect("Failed to fetch users");

    Json(users)
}