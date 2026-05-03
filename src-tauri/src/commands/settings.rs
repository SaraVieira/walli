use crate::errors::AppResult;

#[tauri::command] pub async fn get_settings() -> AppResult<serde_json::Value> { Ok(serde_json::json!({})) }
#[tauri::command] pub async fn update_settings(_patch: serde_json::Value) -> AppResult<serde_json::Value> { Ok(serde_json::json!({})) }
#[tauri::command] pub async fn set_api_key(_source: String, _key: String) -> AppResult<()> { Ok(()) }
#[tauri::command] pub async fn clear_api_key(_source: String) -> AppResult<()> { Ok(()) }
#[tauri::command] pub async fn pick_local_folder() -> AppResult<Option<String>> { Ok(None) }
#[tauri::command] pub async fn set_login_at_startup(_enabled: bool) -> AppResult<()> { Ok(()) }
