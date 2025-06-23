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
    match app_state
        .db
        .add_user_to_group(payload.group_id, user_id)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
