use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{models::group::Group, server::AppState};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetGroupRequest {
    pub owner: u32,
}

#[utoipa::path(
    post,
    path = "/get_groups",
    responses(
        (status = 200, description = "Groups fetched successfully", body = Vec<Group>)
    )
)]

pub async fn get_groups(
    State(app_state): State<AppState>,
    Json(payload): Json<GetGroupRequest>,
) -> Json<Vec<Group>> {
    let groups = app_state.groups.lock().await.clone();
    let owner_groups: Vec<Group> = groups
        .iter()
        .filter(|g| g.owner == payload.owner)
        .cloned()
        .collect();
    Json(owner_groups)
}
