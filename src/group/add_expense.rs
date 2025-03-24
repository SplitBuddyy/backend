// use axum::{extract::State, http::StatusCode, Json};

// use crate::{models::group::ExpenseAddRequest, server::AppState};

// pub async fn root(
//     State(app_state): State<AppState>,
//     Json(payload): Json<ExpenseAddRequest>,
// ) -> Result<Json<bool>, (StatusCode, String)> {
//     let mut groups = app_state.groups.lock().await;

//     if let Some(group) = groups.iter_mut().find(|g| g.owner == payload.group_info.owner && g.id == payload.group_info.group_id) {
//         group.add_expense(payload.expense);
//         return Ok(Json(true)); 
//     } else {
//         Err((StatusCode::NOT_FOUND, "User does not have any group matching those parameters".to_string()))
//     }
// }
