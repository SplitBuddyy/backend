use axum::{extract::State, http::HeaderMap, response::Response, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{auth::utils::extract_user_id_from_headers, models::group::Group, server::AppState};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateGroupRequest {
    pub name: String,
    pub group_start_date: DateTime<Utc>,
    pub group_end_date: DateTime<Utc>,
    pub description: String,
    pub location: String,
}

#[utoipa::path(
    post,
    path = "/create_group",
    request_body = CreateGroupRequest,
    responses(
        (status = 200, description = "Group created successfully", body = String)
    ),
    security(("api_key" = []))
)]
pub async fn create_group(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(group): Json<CreateGroupRequest>,
) -> Response<String> {
    let user_id = match extract_user_id_from_headers(&headers, &app_state).await {
        Ok(id) => id,
        Err(msg) => return Response::new(msg),
    };
    let group = Group::new(
        group.name.as_str(),
        user_id,
        group.group_start_date,
        group.group_end_date,
        group.description,
        group.location,
    );

    match app_state.db.create_group(&group).await {
        Ok(id) => {
            app_state.db.add_user_to_group(id, user_id).await.unwrap();
            Response::new(format!("Group created succesfully: {:?}", id))
        }
        Err(e) => Response::new(e.to_string()),
    }
}
