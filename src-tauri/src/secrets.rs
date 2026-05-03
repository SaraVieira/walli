use crate::errors::{AppError, AppResult};

const SERVICE: &str = "ai.walli.app";

pub fn set(source: &str, key: &str) -> AppResult<()> {
    keyring::Entry::new(SERVICE, source)
        .map_err(|e| AppError::Internal(format!("keyring: {e}")))?
        .set_password(key)
        .map_err(|e| AppError::Internal(format!("keyring set: {e}")))?;
    Ok(())
}

pub fn get(source: &str) -> AppResult<Option<String>> {
    let entry = keyring::Entry::new(SERVICE, source)
        .map_err(|e| AppError::Internal(format!("keyring: {e}")))?;
    match entry.get_password() {
        Ok(v) => Ok(Some(v)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AppError::Internal(format!("keyring get: {e}"))),
    }
}

pub fn clear(source: &str) -> AppResult<()> {
    let entry = keyring::Entry::new(SERVICE, source)
        .map_err(|e| AppError::Internal(format!("keyring: {e}")))?;
    match entry.delete_credential() {
        Ok(_) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(AppError::Internal(format!("keyring delete: {e}"))),
    }
}

pub fn has(source: &str) -> bool { matches!(get(source), Ok(Some(_))) }
