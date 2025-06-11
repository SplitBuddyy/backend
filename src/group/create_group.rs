use axum::{extract::State, response::Response, Json};

use crate::{models::group::Group, server::AppState};

#[utoipa::path(
    post,
    path = "/create_group",
    request_body = Group,
    responses(
        (status = 200, description = "Group created successfully", body = String)
    )
)]
pub async fn create_group(
    State(app_state): State<AppState>,
    Json(group): Json<Group>,
) -> Response<String> {
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

    let owner_id = match app_state
        .users
        .lock()
        .await
        .iter()
        .find(|u| u.id == group.owner_id)
    {
        Some(owner) => owner.id,
        None => return Response::new("Owner does not exist in the database".to_string()),
    };

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
