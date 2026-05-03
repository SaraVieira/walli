use crate::errors::AppResult;

#[tauri::command] pub async fn next_now() -> AppResult<()> { Ok(()) }
#[tauri::command] pub async fn set_paused(_paused: bool) -> AppResult<()> { Ok(()) }
#[tauri::command] pub async fn get_state() -> AppResult<serde_json::Value> { Ok(serde_json::json!({})) }
