use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{models::group::Group, server::AppState};

#[derive(Serialize,Deserialize)]
pub struct GetGroupRequest{
    pub owner:u32
} 

pub async fn root(State(app_state): State<AppState>,Json(payload):Json<GetGroupRequest>) -> Json<Vec<Group>> {
    let groups = app_state.groups.lock().await.clone();
    let owner_groups: Vec<Group> = groups.iter().filter(|g|g.owner==payload.owner).cloned().collect();
    Json(owner_groups)
}
