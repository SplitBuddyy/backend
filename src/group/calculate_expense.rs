use axum::{extract::State, http::StatusCode, Json};

use crate::{
    models::group::{GroupRequest, GroupSummary},
    server::AppState,
};

pub async fn root(
    State(app_state): State<AppState>,
    Json(payload): Json<GroupRequest>,
) -> Result<Json<GroupSummary>, (StatusCode, String)> {
    let groups = app_state.groups.lock().await.clone();
    if let Some(mut group) = groups
        .iter()
        .find(|g| g.owner == payload.owner && g.id == payload.group_id)
        .cloned()
    {
        let summary = group.get_group_summary();
        Ok(Json(summary))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "User does not have any group matching those parameters".to_string(),
        ))
    }
}
