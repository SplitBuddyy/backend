use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Router,
};
use utoipa::OpenApi;

use crate::server::AppState;

mod get_summary;

pub use get_summary::{GroupSummary, Transaction, UserBalance};

#[derive(OpenApi)]
#[openapi(
    paths(get_group_summary),
    components(schemas(GroupSummary, Transaction, UserBalance))
)]
pub struct SummaryApi;

#[utoipa::path(
    get,
    path = "/group/{id}",
    params(
        ("id" = u32, Path, description = "Group ID to get summary for")
    ),
    responses(
        (status = 200, description = "Group summary retrieved successfully", body = GroupSummary),
        (status = 404, description = "Group not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_group_summary(
    State(state): State<AppState>,
    Path(group_id): Path<u32>,
) -> Response<String> {
    match state.db.get_group_summary(group_id).await {
        Ok(summary) => Response::new(serde_json::to_string(&summary).unwrap()),
        Err(e) => Response::new(e.to_string()),
    }
}

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/group/{id}", get(get_group_summary))
        .with_state(app_state)
}
