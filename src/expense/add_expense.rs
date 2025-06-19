use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    auth::utils::extract_user_id_from_headers, models::expenses::ExpenseAddRequest,
    server::AppState,
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
    match app_state.db.create_expense(&payload.expense).await {
        Ok(expense_id) => {
            match app_state
                .db
                .add_participants_to_expense(expense_id, payload.participants_ids)
                .await
            {
                Ok(_) => Ok(Json(true)),
                Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
            }
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
