use axum::{extract::State, http::StatusCode, Json};

use crate::{
    models::group::{get_group_summary, GroupRequest, GroupSummary},
    server::AppState,
};

#[utoipa::path(
    post,
    path = "/calculate",
    request_body = GroupRequest,
    responses(
        (status = 200, description = "Expense calculated successfully", body = GroupSummary)
    )
)]
pub async fn calculate_expense(
    State(app_state): State<AppState>,
    Json(payload): Json<GroupRequest>,
) -> Result<Json<GroupSummary>, (StatusCode, String)> {
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
