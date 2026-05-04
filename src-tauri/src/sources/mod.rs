use crate::errors::AppResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod bing;
pub mod http;
pub mod pool;
pub mod unsplash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind {
    Unsplash,
    Bing,
}

impl SourceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceKind::Unsplash => "unsplash",
            SourceKind::Bing => "bing",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchedImage {
    pub source: SourceKind,
    pub source_id: String,
    pub photographer: Option<String>,
    pub title: Option<String>,
    pub source_url: Option<String>,
    /// Direct URL to download the binary image.
    pub image_url: Option<String>,
    pub download_location: Option<String>, // Unsplash compliance ping URL
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub ext: String, // "jpg", "png", etc.
}

pub struct FetchContext {
    pub tags: Vec<String>,
    pub api_keys: std::collections::HashMap<SourceKind, String>,
    #[allow(dead_code)]
    pub today: String, // YYYY-MM-DD
}

#[async_trait]
pub trait WallpaperSource: Send + Sync {
    #[allow(dead_code)]
    fn kind(&self) -> SourceKind;
    async fn fetch(&self, ctx: &FetchContext) -> AppResult<FetchedImage>;
}
