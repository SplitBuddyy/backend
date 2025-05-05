use axum::{extract::State, response::Response, Json};

use crate::{models::group::Group, server::AppState};

pub async fn root(State(app_state): State<AppState>, Json(group): Json<Group>) -> Response<String> {
    if app_state
        .groups
        .lock()
        .await
        .iter()
        .any(|g| g.name == group.name)
    {
        return Response::new("Group already exists".to_string());
    }
    let id = if app_state.groups.lock().await.len() == 0 {
        0
    } else {
        app_state.groups.lock().await.last().unwrap().id + 1
    };
    let group = Group::new(id, group.name.as_str(), group.owner);
    println!("Group created succesfully: {:?}", group);
    app_state.groups.lock().await.push(group.clone());
    Response::new(format!("Group created succesfully: {:?}", group))
}
