use axum::{extract::State, http::HeaderMap, response::Response, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    auth::utils::extract_user_id_from_headers,
    models::group::Group,
    server::AppState,
};

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

    if app_state
        .groups
        .lock()
        .await
        .iter()
        .any(|g| g.name == group.name)
    {
        return Response::new("Group already exists".to_string());
    }

    let id = if app_state.groups.lock().await.is_empty() {
        0
    } else {
        app_state.groups.lock().await.last().unwrap().id + 1
    };

    let owner_id = user_id;

    let group = Group::new(
        id,
        group.name.as_str(),
        owner_id,
        group.group_start_date,
        group.group_end_date,
        group.description,
        group.location,
    );

    println!("Group created succesfully: {:?}", group);
    app_state.groups.lock().await.push(group.clone());
    Response::new(format!("Group created succesfully: {:?}", group))
}
