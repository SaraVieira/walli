use std::path::Path;
use crate::errors::{AppError, AppResult};

pub fn set_all(path: &Path) -> AppResult<()> {
    let s = path.to_string_lossy().to_string();
    wallpaper::set_from_path(&s).map_err(|e| AppError::Internal(format!("set_from_path: {e}")))?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub mod per_display {
    use std::path::Path;
    use crate::errors::{AppError, AppResult};
    use objc2::rc::Retained;
    use objc2::runtime::AnyObject;
    use objc2::{msg_send, ClassType};
    use objc2_foundation::{NSArray, NSDictionary, NSError, NSString, NSURL};
    use objc2_app_kit::{NSScreen, NSWorkspace};

    pub fn screen_ids() -> AppResult<Vec<String>> {
        unsafe {
            let mtm = objc2_foundation::MainThreadMarker::new()
                .ok_or_else(|| AppError::Internal("must be called from main thread".into()))?;
            let screens: Retained<NSArray<NSScreen>> = NSScreen::screens(mtm);
            Ok((0..screens.count()).map(|i| format!("display-{i}")).collect())
        }
    }

    pub fn set_for_display(index: usize, path: &Path) -> AppResult<()> {
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
                    let ns_err: Retained<NSError> = Retained::from_raw(err as *mut NSError)
                        .expect("non-null NSError pointer");
                    let desc = ns_err.localizedDescription();
                    return Err(AppError::Internal(format!(
                        "setDesktopImageURL failed: {}",
                        desc.to_string()
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
    use std::path::Path;
    use crate::errors::AppResult;
    pub fn screen_ids() -> AppResult<Vec<String>> { Ok(vec!["display-0".into()]) }
    pub fn set_for_display(_i: usize, p: &Path) -> AppResult<()> { super::set_all(p) }
}
