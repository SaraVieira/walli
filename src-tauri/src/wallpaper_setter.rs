use crate::errors::{AppError, AppResult};
use std::path::Path;

pub fn set_all(path: &Path) -> AppResult<()> {
    tracing::debug!(path = ?path, "setting wallpaper on all displays");
    let s = path.to_string_lossy().to_string();
    match wallpaper::set_from_path(&s) {
        Ok(()) => {
            tracing::debug!(path = ?path, "wallpaper crate set_from_path ok");
            Ok(())
        }
        Err(e) => {
            tracing::warn!(path = ?path, ?e, "wallpaper crate set_from_path failed");
            Err(AppError::Internal(format!("set_from_path: {e}")))
        }
    }
}

#[cfg(target_os = "macos")]
pub mod per_display {
    use crate::errors::{AppError, AppResult};
    use objc2::msg_send;
    use objc2::rc::Retained;
    use objc2::runtime::AnyObject;
    use objc2_app_kit::{NSScreen, NSWorkspace};
    use objc2_foundation::{NSArray, NSDictionary, NSError, NSString, NSURL};
    use std::path::Path;

    pub fn screen_ids() -> AppResult<Vec<String>> {
        let mtm = objc2_foundation::MainThreadMarker::new()
            .ok_or_else(|| AppError::Internal("must be called from main thread".into()))?;
        let screens: Retained<NSArray<NSScreen>> = NSScreen::screens(mtm);
        Ok((0..screens.count())
            .map(|i| format!("display-{i}"))
            .collect())
    }

    pub fn set_for_display(index: usize, path: &Path) -> AppResult<()> {
        tracing::debug!(index, path = ?path, "setting wallpaper for display");
        unsafe {
            let mtm = objc2_foundation::MainThreadMarker::new()
                .ok_or_else(|| AppError::Internal("must be called from main thread".into()))?;
            let screens = NSScreen::screens(mtm);
            let count = screens.count();
            if index >= count {
                return Err(AppError::Invalid(format!(
                    "display index {index} out of range, only {count} screens"
                )));
            }
            let screen = screens.objectAtIndex(index);
            let path_str = path.to_string_lossy();
            let url = NSURL::fileURLWithPath(&NSString::from_str(&path_str));
            let workspace = NSWorkspace::sharedWorkspace();
            let opts: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::new();
            let mut err: *mut AnyObject = std::ptr::null_mut();
            let ok: bool = msg_send![&workspace,
                setDesktopImageURL: &*url,
                forScreen: &*screen,
                options: &*opts,
                error: &mut err];
            if !ok {
                if !err.is_null() {
                    let ns_err: Retained<NSError> =
                        Retained::from_raw(err as *mut NSError).expect("non-null NSError pointer");
                    let desc = ns_err.localizedDescription();
                    return Err(AppError::Internal(format!(
                        "setDesktopImageURL failed: {desc}"
                    )));
                }
                return Err(AppError::Internal("setDesktopImageURL failed".into()));
            }
            Ok(())
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub mod per_display {
    use crate::errors::AppResult;
    use std::path::Path;
    pub fn screen_ids() -> AppResult<Vec<String>> {
        Ok(vec!["display-0".into()])
    }
    pub fn set_for_display(index: usize, path: &Path) -> AppResult<()> {
        tracing::debug!(index, path = ?path, "setting wallpaper for display");
        super::set_all(path)
    }
}
