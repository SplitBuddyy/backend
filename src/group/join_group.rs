use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    auth::utils::extract_user_id_from_headers, models::group::JoinGroupRequest, server::AppState,
};

#[utoipa::path(
    post,
    path = "/join_group",
    request_body = JoinGroupRequest,
    responses(
        (status = 200, description = "User added to group successfully", body = bool)
    ),
    security(("api_key" = []))
)]
pub async fn join_group(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<JoinGroupRequest>,
) -> Result<Json<bool>, (StatusCode, String)> {
    let user_id = match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(id) => id,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string())),
    };
    let mut groups = app_state.groups.lock().await;
    let users = app_state.users.lock().await;
    let user_exist = users.iter().find(|u| u.id == Some(user_id));

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
        if group.members_ids.contains(&user.id.unwrap()) {
            return Err((
                StatusCode::BAD_REQUEST,
                "User already in the group".to_string(),
            ));
        }
        group.add_members(user.id.unwrap());
        Ok(Json(true))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "User does not have any group matching those parameters".to_string(),
        ))
    }
}
