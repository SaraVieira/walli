use crate::errors::AppResult;

#[tauri::command] pub async fn list_collections() -> AppResult<serde_json::Value> { Ok(serde_json::json!([])) }
#[tauri::command] pub async fn create_collection(_name: String, _tags: Vec<String>) -> AppResult<serde_json::Value> { Ok(serde_json::json!({})) }
#[tauri::command] pub async fn update_collection(_id: i64, _name: String, _tags: Vec<String>) -> AppResult<serde_json::Value> { Ok(serde_json::json!({})) }
#[tauri::command] pub async fn delete_collection(_id: i64) -> AppResult<()> { Ok(()) }
#[tauri::command] pub async fn set_active_collection(_id: i64) -> AppResult<()> { Ok(()) }
