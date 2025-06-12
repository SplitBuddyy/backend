use crate::server::AppState;

pub async fn get_user_id_by_api_key(api_key: &str, app_state: &AppState) -> Option<u32> {
    let api_tokens = app_state.api_tokens.lock().await;
    if let Some(user_id) = api_tokens.get(api_key) {
        Some(*user_id)
    } else {
        None
    }
}