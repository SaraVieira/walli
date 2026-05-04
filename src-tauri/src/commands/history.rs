use crate::db::{queries, Pool};
use crate::errors::AppResult;
use crate::scheduler::{SchedulerHandle, SchedulerMsg};
use crate::wallpaper_setter;
use chrono::Utc;
use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
pub async fn list_history(
    app: AppHandle,
    limit: u32,
    offset: u32,
    favorites_only: bool,
) -> AppResult<Vec<queries::HistoryEntry>> {
    let pool = app.state::<Pool>().inner().clone();
    queries::list_history(&pool, limit, offset, favorites_only)
}

#[tauri::command]
pub async fn toggle_favorite(app: AppHandle, wallpaper_id: i64) -> AppResult<bool> {
    let pool = app.state::<Pool>().inner().clone();
    let new_state = queries::toggle_favorite(&pool, wallpaper_id, Utc::now().timestamp())?;
    if let Some(w) = queries::get_wallpaper(&pool, wallpaper_id)? {
        let _ = app.emit("wallpaper-changed", &w);
    }
    Ok(new_state)
}

#[tauri::command]
pub async fn set_wallpaper_from_history(app: AppHandle, wallpaper_id: i64) -> AppResult<()> {
    tracing::info!(wallpaper_id, "set_wallpaper_from_history");
    let pool = app.state::<Pool>().inner().clone();
    let w =
        queries::get_wallpaper(&pool, wallpaper_id)?.ok_or(crate::errors::AppError::NotFound)?;
    wallpaper_setter::set_all(std::path::Path::new(&w.file_path))?;
    queries::record_history(&pool, w.id, Utc::now().timestamp(), None)?;
    let _ = app.emit("wallpaper-changed", &w);
    if let Some(h) = app.try_state::<SchedulerHandle>() {
        let _ = h.tx.send(SchedulerMsg::Reschedule).await;
    }
    Ok(())
}
#[tauri::command]
pub async fn open_history_window(app: AppHandle) -> AppResult<()> {
    if let Some(w) = app.get_webview_window("history") {
        let _ = w.show();
        let _ = w.set_focus();
    }
    Ok(())
}
