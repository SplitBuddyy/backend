use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    auth::utils::extract_user_id_from_headers,
    models::expenses::{Expense, GetExpensesByUserIdRequest},
    server::AppState,
};

#[utoipa::path(
    post,
    path = "/get_all_user_expenses",
    request_body = GetExpensesByUserIdRequest,
    responses(
        (status = 200, description = "Expenses fetched successfully", body = Vec<Expense>)
    ),
    security(("api_key" = []))
)]
pub async fn get_all_user_expenses(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<GetExpensesByUserIdRequest>,
) -> Result<(StatusCode, Json<Vec<Expense>>), (StatusCode, String)> {
    match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(_) => (),
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string())),
    };
    let expenses_ids = app_state
        .db
        .get_all_user_expenses(payload.user_id)
        .await
        .unwrap();
    let expenses = app_state
        .db
        .get_expenses_by_ids(expenses_ids)
        .await
        .unwrap();
    Ok((StatusCode::OK, Json(expenses)))
}
