use axum::{extract::State, http::HeaderMap, response::Response, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{auth::utils::get_user_id_by_api_key, models::group::Group, server::AppState};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateGroupRequest {
   pub name: String,
   pub group_start_date: DateTime<Utc>,
   pub group_end_date: DateTime<Utc>,
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
    let api_key = headers.get("todo_apikey").unwrap().to_str().unwrap();
    let user_id = get_user_id_by_api_key(api_key, &app_state).await;
    if user_id.is_none() {
        return Response::new("Invalid API key".to_string());
    }
    let user_id = user_id.unwrap();

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
    );

    println!("Group created succesfully: {:?}", group);
    app_state.groups.lock().await.push(group.clone());
    Response::new(format!("Group created succesfully: {:?}", group))
}
