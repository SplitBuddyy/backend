use axum::{extract::State, http::StatusCode, Json};

use crate::{models::group::AddMemberRequest, server::AppState};

#[utoipa::path(
    post,
    path = "/add_to_group",
    request_body = AddMemberRequest,
    responses(
        (status = 200, description = "User added to group successfully", body = bool)
    )
)]
pub async fn add_to_group(
    State(app_state): State<AppState>,
    Json(payload): Json<AddMemberRequest>,
) -> Result<Json<bool>, (StatusCode, String)> {
    let mut groups = app_state.groups.lock().await;

    if let Some(group) = groups
        .iter_mut()
        .find(|g| g.owner == payload.group_info.owner && g.id == payload.group_info.group_id)
    {
        group.add_members(payload.member);
        Ok(Json(true))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "User does not have any group matching those parameters".to_string(),
        ))
    }
}
