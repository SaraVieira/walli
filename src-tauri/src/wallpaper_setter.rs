use crate::errors::{AppError, AppResult};
use std::path::{Path, PathBuf};
use tauri::AppHandle;

pub async fn set_all_on_main(app: &AppHandle, path: &Path) -> AppResult<()> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let path: PathBuf = path.to_path_buf();
    app.run_on_main_thread(move || {
        let _ = tx.send(set_all_blocking(&path));
    })
    .map_err(|e| AppError::Internal(format!("run_on_main_thread: {e}")))?;
    rx.await
        .map_err(|e| AppError::Internal(format!("main-thread reply dropped: {e}")))?
}

fn set_all_blocking(path: &Path) -> AppResult<()> {
    tracing::debug!(path = ?path, "setting wallpaper");
    let s = path.to_string_lossy().to_string();
    wallpaper::set_from_path(&s).map_err(|e| AppError::Internal(format!("set_from_path: {e}")))
}
