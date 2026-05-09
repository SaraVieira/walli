use crate::db::{queries, Pool};
use crate::errors::{AppError, AppResult};
use crate::scheduler::{SchedulerHandle, SchedulerMsg};
use crate::sources::SourceKind;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsDto {
    pub interval_seconds: u64,
    pub paused: bool,
    pub active_collection_id: Option<i64>,
    pub source_unsplash_enabled: bool,
    pub source_bing_enabled: bool,
    pub unsplash_key_set: bool,
    pub login_at_startup: bool,
}

fn b(map: &HashMap<String, String>, k: &str) -> bool {
    map.get(k).map(String::as_str) == Some("true")
}

const WRITABLE_KEYS: &[&str] = &[
    "interval_seconds",
    "paused",
    "active_collection_id",
    "source_unsplash_enabled",
    "source_bing_enabled",
];

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> AppResult<SettingsDto> {
    let pool = app.state::<Pool>().inner().clone();
    let s = queries::get_settings(&pool).await?;
    let autostart = app.autolaunch().is_enabled().unwrap_or(false);
    Ok(SettingsDto {
        interval_seconds: s
            .get("interval_seconds")
            .and_then(|x| x.parse().ok())
            .unwrap_or(3600),
        paused: b(&s, "paused"),
        active_collection_id: s.get("active_collection_id").and_then(|x| x.parse().ok()),
        source_unsplash_enabled: b(&s, "source_unsplash_enabled"),
        source_bing_enabled: b(&s, "source_bing_enabled"),
        unsplash_key_set: s
            .get("unsplash_api_key")
            .map(|v| !v.is_empty())
            .unwrap_or(false),
        login_at_startup: autostart,
    })
}

#[tauri::command]
pub async fn update_settings(app: AppHandle, patch: serde_json::Value) -> AppResult<SettingsDto> {
    tracing::info!(?patch, "updating settings");
    let pool = app.state::<Pool>().inner().clone();
    let m = patch
        .as_object()
        .ok_or_else(|| AppError::Invalid("patch must be object".into()))?;
    for (k, v) in m {
        if !WRITABLE_KEYS.contains(&k.as_str()) {
            return Err(AppError::Invalid(format!("setting '{k}' is not writable")));
        }
        let value_str = match v {
            serde_json::Value::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Null => "".into(),
            _ => continue,
        };
        queries::set_setting(&pool, k, &value_str).await?;
    }
    if let Some(h) = app.try_state::<SchedulerHandle>() {
        let _ = h.tx.send(SchedulerMsg::Reschedule).await;
    }
    let _ = app.emit("settings-changed", ());
    get_settings(app).await
}

#[tauri::command]
pub async fn set_api_key(app: AppHandle, source: SourceKind, key: String) -> AppResult<()> {
    tracing::info!(?source, "set_api_key (length={})", key.len());
    let pool = app.state::<Pool>().inner().clone();
    queries::set_setting(&pool, &format!("{}_api_key", source.as_str()), &key).await?;
    Ok(())
}

#[tauri::command]
pub async fn clear_api_key(app: AppHandle, source: SourceKind) -> AppResult<()> {
    tracing::info!(?source, "clear_api_key");
    let pool = app.state::<Pool>().inner().clone();
    queries::set_setting(&pool, &format!("{}_api_key", source.as_str()), "").await?;
    Ok(())
}

#[tauri::command]
pub async fn set_login_at_startup(app: AppHandle, enabled: bool) -> AppResult<()> {
    let m = app.autolaunch();
    if enabled {
        let _ = m.enable();
    } else {
        let _ = m.disable();
    }
    Ok(())
}

#[tauri::command]
pub async fn open_settings_window(app: AppHandle) -> AppResult<()> {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.show();
        let _ = w.set_focus();
    }
    Ok(())
}
