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
) -> AppResult<Vec<queries::HistoryEntry>> {
    let pool = app.state::<Pool>().inner().clone();
    queries::list_history(&pool, limit, offset).await
}

#[tauri::command]
pub async fn set_wallpaper_from_history(app: AppHandle, wallpaper_id: i64) -> AppResult<()> {
    tracing::info!(wallpaper_id, "set_wallpaper_from_history");
    let pool = app.state::<Pool>().inner().clone();
    let w = queries::get_wallpaper(&pool, wallpaper_id)
        .await?
        .ok_or(crate::errors::AppError::NotFound)?;
    let s = queries::get_settings(&pool).await?;
    let per_display = s.get("per_display_mode").map(String::as_str) == Some("true");
    let now = Utc::now().timestamp();
    if per_display {
        let ids = wallpaper_setter::screen_ids_on_main(&app).await?;
        for (i, sid) in ids.iter().enumerate() {
            wallpaper_setter::set_for_display_on_main(&app, i, std::path::Path::new(&w.file_path))
                .await?;
            queries::record_history(&pool, w.id, now, Some(sid.as_str())).await?;
        }
    } else {
        wallpaper_setter::set_all_on_main(&app, std::path::Path::new(&w.file_path)).await?;
        queries::record_history(&pool, w.id, now, None).await?;
    }
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
