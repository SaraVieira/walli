use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("io: {0}")]
    Io(String),
    #[error("db: {0}")]
    Db(String),
    #[error("http {status:?}: {message}")]
    Http {
        status: Option<u16>,
        message: String,
    },
    #[error("not found")]
    NotFound,
    #[error("invalid: {0}")]
    Invalid(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl AppError {
    /// Returns false for HTTP responses that won't change on retry
    /// (4xx except 408 Request Timeout and 429 Too Many Requests).
    pub fn is_retryable(&self) -> bool {
        match self {
            AppError::Http {
                status: Some(s), ..
            } => !(*s >= 400 && *s < 500 && *s != 408 && *s != 429),
            _ => true,
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        Self::Internal(e.to_string())
    }
}
impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(e.to_string())
    }
}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http {
            status: e.status().map(|s| s.as_u16()),
            message: e.to_string(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
