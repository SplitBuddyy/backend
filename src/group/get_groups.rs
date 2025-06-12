use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{models::group::Group, server::AppState};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetGroupRequest {
    pub user_id: u32,
}

#[utoipa::path(
    post,
    path = "/get_user_groups",
    responses(
        (status = 200, description = "Groups fetched successfully", body = Vec<Group>)
    )
)]

pub async fn get_user_groups(
    State(app_state): State<AppState>,
    Json(payload): Json<GetGroupRequest>,
) -> Json<Vec<Group>> {
    let groups = app_state.groups.lock().await.clone();
    let user_groups: Vec<Group> = groups
        .iter()
        .filter(|g| g.members_ids.contains(&payload.user_id))
        .cloned()
        .collect();
    Json(user_groups)
}
