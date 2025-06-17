use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    auth::utils::extract_user_id_from_headers, models::group::ExpenseAddRequest, server::AppState,
};

#[utoipa::path(
    post,
    path = "/add_expense",
    request_body = ExpenseAddRequest,
    responses(
        (status = 200, description = "Expense added to group successfully", body = bool)
    ),
    security(("api_key" = []))
)]
pub async fn add_expense(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ExpenseAddRequest>,
) -> Result<Json<bool>, (StatusCode, String)> {
    match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(_) => (),
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string())),
    };
    let mut groups = app_state.groups.lock().await;

    if let Some(group) = groups.iter_mut().find(|g| g.id == payload.group_id) {
        group.add_expense(payload.expense);
        Ok(Json(true))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "User does not have any group matching those parameters".to_string(),
        ))
    }
}
