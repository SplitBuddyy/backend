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
    let users = app_state.users.lock().await;
    let user_exist = users.iter().find(|u| u.id == payload.member_id);

    let user = match user_exist {
        Some(user) => user.clone(),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "User does not exist in the database".to_string(),
            ));
        }
    };

    if let Some(group) = groups.iter_mut().find(|g| g.id == payload.group_id) {
        if group.members_ids.contains(&user.id) {
            return Err((
                StatusCode::BAD_REQUEST,
                "User already in the group".to_string(),
            ));
        }
        group.add_members(user.id);
        Ok(Json(true))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "User does not have any group matching those parameters".to_string(),
        ))
    }
}
