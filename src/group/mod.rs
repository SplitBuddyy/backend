use crate::server::AppState;
use axum::{routing::post, Router};

mod join_group;
use join_group::__path_join_group;
use join_group::join_group;

mod create_group;
use create_group::__path_create_group;
use create_group::create_group;

mod get_owned_groups;
use get_owned_groups::__path_get_user_owned_groups;
use get_owned_groups::get_user_owned_groups;

mod get_joined_groups;
use get_joined_groups::__path_get_user_joined_groups;
use get_joined_groups::get_user_joined_groups;

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(paths(
    get_user_owned_groups,
    create_group,
    join_group,
    get_user_joined_groups,
))]
pub struct GroupApi;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/create_group", post(create_group))
        .route("/get_user_owned_groups", post(get_user_owned_groups))
        .route("/get_user_joined_groups", post(get_user_joined_groups))
        .route("/join_group", post(join_group))
        .with_state(app_state)
}
