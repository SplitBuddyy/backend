use axum::http::HeaderMap;

use crate::server::AppState;

pub async fn extract_user_id_from_headers(
    headers: &HeaderMap,
    app_state: &AppState,
) -> Result<u32, String> {
    let token = headers
        .get("todo_apikey")
        .ok_or_else(|| "Missing API key header".to_string())?
        .to_str()
        .map_err(|_| "Invalid API key header format".to_string())?;
    let user_id = app_state.db.get_user_id_by_token(&token).await.unwrap();
    Ok(user_id)
}
