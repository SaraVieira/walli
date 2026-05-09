use crate::db::{queries, Pool};
use crate::errors::AppResult;
use crate::scheduler::{SchedulerHandle, SchedulerMsg};
use chrono::Utc;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn list_collections(app: AppHandle) -> AppResult<Vec<queries::Collection>> {
    let pool = app.state::<Pool>().inner().clone();
    queries::list_collections(&pool).await
}

#[tauri::command]
pub async fn create_collection(
    app: AppHandle,
    name: String,
    tags: Vec<String>,
) -> AppResult<queries::Collection> {
    let pool = app.state::<Pool>().inner().clone();
    queries::create_collection(&pool, &name, &tags, Utc::now().timestamp()).await
}

#[tauri::command]
pub async fn update_collection(
    app: AppHandle,
    id: i64,
    name: String,
    tags: Vec<String>,
) -> AppResult<queries::Collection> {
    let pool = app.state::<Pool>().inner().clone();
    queries::update_collection(&pool, id, &name, &tags).await
}

#[tauri::command]
pub async fn delete_collection(app: AppHandle, id: i64) -> AppResult<()> {
    let pool = app.state::<Pool>().inner().clone();
    queries::delete_collection(&pool, id).await
}

#[tauri::command]
pub async fn set_active_collection(app: AppHandle, id: i64) -> AppResult<()> {
    let pool = app.state::<Pool>().inner().clone();
    queries::set_setting(&pool, "active_collection_id", &id.to_string()).await?;
    if let Some(h) = app.try_state::<SchedulerHandle>() {
        let _ = h.tx.send(SchedulerMsg::NextNow).await;
    }
    Ok(())
}
