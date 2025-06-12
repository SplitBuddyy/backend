use axum::http::HeaderMap;

use crate::server::AppState;

pub async fn get_user_id_by_api_key(api_key: &str, app_state: &AppState) -> Option<u32> {
    let api_tokens = app_state.api_tokens.lock().await;
    api_tokens.get(api_key).copied()
}

pub async fn extract_user_id_from_headers(
    headers: &HeaderMap,
    app_state: &AppState,
) -> Result<u32, String> {
    let api_key = headers
        .get("todo_apikey")
        .ok_or_else(|| "Missing API key header".to_string())?
        .to_str()
        .map_err(|_| "Invalid API key header format".to_string())?;
    let user_id = get_user_id_by_api_key(api_key, app_state).await;
    user_id.ok_or_else(|| "Invalid API key".to_string())
}
