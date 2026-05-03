use crate::errors::AppResult;

#[tauri::command] pub async fn list_history(_limit: u32, _offset: u32, _favorites_only: bool) -> AppResult<serde_json::Value> { Ok(serde_json::json!([])) }
#[tauri::command] pub async fn toggle_favorite(_wallpaper_id: i64) -> AppResult<bool> { Ok(false) }
#[tauri::command] pub async fn set_wallpaper_from_history(_wallpaper_id: i64) -> AppResult<()> { Ok(()) }
