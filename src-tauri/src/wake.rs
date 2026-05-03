use tauri::{AppHandle, Emitter};

pub fn install(app: AppHandle) {
    #[cfg(target_os = "macos")]
    {
        std::thread::spawn(move || {
            // Polling fallback: detect monotonic clock jumps as a wake signal.
            // (Subscribing to NSWorkspaceDidWakeNotification via objc2 is preferred but
            // fiddly; this approximation is sufficient and side-effect-free.)
            let mut last = std::time::Instant::now();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(30));
                let now = std::time::Instant::now();
                let elapsed = now.duration_since(last);
                last = now;
                if elapsed > std::time::Duration::from_secs(120) {
                    let _ = app.emit("wake", ());
                }
            }
        });
    }
    #[cfg(not(target_os = "macos"))]
    { let _ = app; }
}
