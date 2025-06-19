use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    auth::utils::extract_user_id_from_headers,
    models::expenses::{Expense, GetExpensesByGroupIdRequest},
    server::AppState,
};

#[utoipa::path(
    post,
    path = "/get_group_expenses",
    request_body = GetExpensesByGroupIdRequest,
    responses(
        (status = 200, description = "Expenses fetched successfully", body = Vec<Expense>)
    ),
    security(("api_key" = []))
)]
pub async fn get_group_expenses(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<GetExpensesByGroupIdRequest>,
) -> Result<(StatusCode, Json<Vec<Expense>>), (StatusCode, String)> {
    match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(_) => (),
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string())),
    };
    match app_state
        .db
        .get_expenses_by_group_id(payload.group_id)
        .await
    {
        Ok(expenses) => Ok((StatusCode::OK, Json(expenses))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
