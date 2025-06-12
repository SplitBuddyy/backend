use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};

use crate::{
    auth::utils::extract_user_id_from_headers, models::group::{get_group_summary, GroupRequest, GroupSummary}, server::AppState
};

#[utoipa::path(
    post,
    path = "/calculate",
    request_body = GroupRequest,
    responses(
        (status = 200, description = "Expense calculated successfully", body = GroupSummary)
    ),
    security(("api_key" = []))
)]
pub async fn calculate_expense(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<GroupRequest>,
) -> Result<Json<GroupSummary>, (StatusCode, String)> {
    let _ = match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(_) => (),
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string())),
    };

    let groups = app_state.groups.lock().await.clone();
    if let Some(group) = groups.iter().find(|g| g.id == payload.group_id).cloned() {
        let summary = get_group_summary(group);
        Ok(Json(summary))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "User does not have any group matching those parameters".to_string(),
        ))
    }
}
