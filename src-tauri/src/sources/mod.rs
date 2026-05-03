use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::errors::AppResult;

pub mod http;
pub mod apod;
pub mod bing;
pub mod local;
pub mod pool;
pub mod unsplash;
pub mod wallhaven;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind { Unsplash, Wallhaven, Bing, Apod, Local }

impl SourceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceKind::Unsplash => "unsplash",
            SourceKind::Wallhaven => "wallhaven",
            SourceKind::Bing => "bing",
            SourceKind::Apod => "apod",
            SourceKind::Local => "local",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchedImage {
    pub source: SourceKind,
    pub source_id: String,
    pub photographer: Option<String>,
    pub source_url: Option<String>,
    /// Direct URL to download the binary image (None for local).
    pub image_url: Option<String>,
    /// For local source, the absolute file path.
    pub local_path: Option<String>,
    pub download_location: Option<String>, // Unsplash compliance ping URL
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub ext: String, // "jpg", "png", etc.
}

pub struct FetchContext {
    pub tags: Vec<String>,
    pub api_keys: std::collections::HashMap<SourceKind, String>,
    pub local_folder: Option<String>,
    pub today: String, // YYYY-MM-DD
}

#[async_trait]
pub trait WallpaperSource: Send + Sync {
    fn kind(&self) -> SourceKind;
    async fn fetch(&self, ctx: &FetchContext) -> AppResult<FetchedImage>;
}
