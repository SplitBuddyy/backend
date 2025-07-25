use crate::{auth::utils::extract_user_id_from_headers, models::group::Group, server::AppState};
use axum::{extract::State, http::HeaderMap, Json};

#[utoipa::path(
    post,
    path = "/get_user_owned_groups",
    responses(
        (status = 200, description = "Groups fetched successfully", body = Vec<Group>)
    ),
    security(("api_key" = []))
)]

pub async fn get_user_owned_groups(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Json<Vec<Group>> {
    let user_id = match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(id) => id,
        Err(_) => return Json(vec![]),
    };
    let groups = app_state.db.get_groups_by_owner_id(user_id).await.unwrap();
    Json(groups)
}
