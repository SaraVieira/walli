use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager, WebviewWindow,
};
use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

use crate::db::{queries, Pool};
use crate::scheduler::{SchedulerHandle, SchedulerMsg};

static LAST_SHOWN_MS: AtomicI64 = AtomicI64::new(0);
const FOCUS_GRACE_MS: i64 = 400;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Bring the (Accessory-mode) app to the foreground so the popover can actually take focus.
#[cfg(target_os = "macos")]
fn activate_app() {
    use objc2_app_kit::NSApplication;
    use objc2_foundation::MainThreadMarker;
    if let Some(mtm) = MainThreadMarker::new() {
        let ns_app = NSApplication::sharedApplication(mtm);
        #[allow(deprecated)]
        ns_app.activateIgnoringOtherApps(true);
    }
}

#[cfg(not(target_os = "macos"))]
fn activate_app() {}

pub fn install(app: &App) -> tauri::Result<()> {
    let icon = Image::from_bytes(include_bytes!("../icons/tray.png"))?;

    let next_item = MenuItem::with_id(app, "next", "Next wallpaper", true, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause", "Pause", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings…", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&next_item, &pause_item, &sep, &settings_item, &quit_item],
    )?;

    let _ = TrayIconBuilder::with_id("walli-tray")
        .icon(icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "next" => {
                if let Some(h) = app.try_state::<SchedulerHandle>() {
                    let tx = h.tx.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = tx.send(SchedulerMsg::NextNow).await;
                    });
                }
            }
            "pause" => {
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    let pool = match app.try_state::<Pool>() {
                        Some(p) => p.inner().clone(),
                        None => return,
                    };
                    let s = match queries::get_settings(&pool) {
                        Ok(s) => s,
                        Err(_) => return,
                    };
                    let currently_paused =
                        s.get("paused").map(String::as_str) == Some("true");
                    let new_val = if currently_paused { "false" } else { "true" };
                    if queries::set_setting(&pool, "paused", new_val).is_err() {
                        return;
                    }
                    if let Some(h) = app.try_state::<SchedulerHandle>() {
                        let _ = h.tx.send(SchedulerMsg::Reschedule).await;
                    }
                });
            }
            "settings" => {
                if let Some(w) = app.get_webview_window("settings") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            on_tray_event(tray.app_handle(), &event);
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(win) = app.get_webview_window("popover") {
                    toggle(&win);
                }
            }
        })
        .build(app)?;
    // Auto-hide on focus loss — disabled while iterating on UI so the popover
    // stays open while the dev console / editor takes focus. Re-enable for
    // production behavior.
    if let Some(popover) = app.get_webview_window("popover") {
        let popover_clone = popover.clone();
        popover.on_window_event(move |e| {
            match e {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = popover_clone.hide();
                }
                tauri::WindowEvent::Focused(false) if !cfg!(debug_assertions) => {
                    let elapsed = now_ms() - LAST_SHOWN_MS.load(Ordering::Relaxed);
                    if elapsed > FOCUS_GRACE_MS {
                        let _ = popover_clone.hide();
                    }
                }
                _ => {}
            }
        });
    }
    if let Some(settings) = app.get_webview_window("settings") {
        let settings_clone = settings.clone();
        settings.on_window_event(move |e| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                api.prevent_close();
                let _ = settings_clone.hide();
            }
        });
    }
    Ok(())
}

fn toggle(win: &WebviewWindow) {
    if win.is_visible().unwrap_or(false) {
        let _ = win.hide();
    } else {
        let _ = win.move_window(Position::TrayCenter);
        let _ = win.show();
        activate_app();
        let _ = win.set_focus();
        LAST_SHOWN_MS.store(now_ms(), Ordering::Relaxed);
    }
}
