use crate::db::{queries, Pool};
use crate::errors::AppResult;
use crate::scheduler::{SchedulerHandle, SchedulerMsg};
use tauri::{AppHandle, Manager};

#[derive(serde::Serialize)]
pub struct AppState {
    pub current: Option<queries::Wallpaper>,
    pub paused: bool,
    pub interval_seconds: u64,
    pub active_collection_id: Option<i64>,
    pub error_banner: Option<String>,
}

#[tauri::command]
pub async fn get_state(app: AppHandle) -> AppResult<AppState> {
    let pool = app.state::<Pool>().inner().clone();
    let s = queries::get_settings(&pool)?;
    let current_id = queries::list_history(&pool, 1, 0, false)?
        .into_iter()
        .next()
        .map(|h| h.wallpaper.id);
    let current = if let Some(id) = current_id {
        queries::get_wallpaper(&pool, id)?
    } else {
        None
    };
    Ok(AppState {
        current,
        paused: s.get("paused").map(String::as_str) == Some("true"),
        interval_seconds: s
            .get("interval_seconds")
            .and_then(|x| x.parse().ok())
            .unwrap_or(3600),
        active_collection_id: s.get("active_collection_id").and_then(|x| x.parse().ok()),
        error_banner: None,
    })
}

#[tauri::command]
pub async fn next_now(app: AppHandle) -> AppResult<()> {
    tracing::info!("next_now requested");
    if let Some(h) = app.try_state::<SchedulerHandle>() {
        let _ = h.tx.send(SchedulerMsg::NextNow).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn set_paused(app: AppHandle, paused: bool) -> AppResult<()> {
    tracing::info!(paused, "set_paused");
    let pool = app.state::<Pool>().inner().clone();
    queries::set_setting(&pool, "paused", if paused { "true" } else { "false" })?;
    if let Some(h) = app.try_state::<SchedulerHandle>() {
        let _ = h.tx.send(SchedulerMsg::Reschedule).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: AppHandle) -> AppResult<()> {
    app.exit(0);
    Ok(())
}
