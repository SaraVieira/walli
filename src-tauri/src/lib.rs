use tauri::{ActivationPolicy, Manager};

mod cache;
mod commands;
mod db;
mod errors;
mod scheduler;
mod sources;
mod tray;
mod wake;
mod wallpaper_setter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter("info,walli=debug")
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            tray::install(app)?;

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = bootstrap(handle).await {
                    tracing::error!(?e, "bootstrap failed");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::control::next_now,
            commands::control::set_paused,
            commands::control::get_state,
            commands::control::quit_app,
            commands::collections::list_collections,
            commands::collections::create_collection,
            commands::collections::update_collection,
            commands::collections::delete_collection,
            commands::collections::set_active_collection,
            commands::history::list_history,
            commands::history::toggle_favorite,
            commands::history::set_wallpaper_from_history,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::set_api_key,
            commands::settings::clear_api_key,
            commands::settings::pick_local_folder,
            commands::settings::set_login_at_startup,
            commands::settings::open_settings_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn bootstrap(app: tauri::AppHandle) -> anyhow::Result<()> {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    let pool = db::init(&app_dir.join("walli.db")).await?;
    app.manage(pool.clone());

    let cache = cache::Cache::new(app_dir.join("wallpapers"))?;
    app.manage(cache);

    let scheduler_handle = scheduler::start(app.clone()).await?;
    app.manage(scheduler_handle);
    wake::install(app.clone());
    Ok(())
}
