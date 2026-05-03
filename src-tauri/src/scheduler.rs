use tauri::AppHandle;

#[derive(Clone)]
pub struct SchedulerHandle;

pub async fn start(_app: AppHandle) -> anyhow::Result<SchedulerHandle> { Ok(SchedulerHandle) }
