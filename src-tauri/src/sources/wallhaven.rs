use async_trait::async_trait;
use rand::seq::SliceRandom;
use serde::Deserialize;
use crate::errors::{AppError, AppResult};
use super::{http::HTTP, FetchContext, FetchedImage, SourceKind, WallpaperSource};

pub struct Wallhaven;

#[derive(Deserialize)]
struct WhResponse { data: Vec<WhPhoto> }
#[derive(Deserialize)]
struct WhPhoto {
    id: String,
    path: String,           // direct image URL
    url: String,            // wallpaper page URL
    file_type: String,      // e.g. "image/jpeg"
    dimension_x: i64,
    dimension_y: i64,
}

#[async_trait]
impl WallpaperSource for Wallhaven {
    fn kind(&self) -> SourceKind { SourceKind::Wallhaven }

    async fn fetch(&self, ctx: &FetchContext) -> AppResult<FetchedImage> {
        let tag = ctx.tags.choose(&mut rand::thread_rng())
            .ok_or_else(|| AppError::Invalid("Active collection has no tags".into()))?;
        let mut req = HTTP.get("https://wallhaven.cc/api/v1/search")
            .query(&[
                ("q", tag.as_str()),
                ("categories", "110"),
                ("purity", "100"),
                ("sorting", "random"),
                ("atleast", "1920x1080"),
            ]);
        if let Some(key) = ctx.api_keys.get(&SourceKind::Wallhaven) {
            req = req.header("X-API-Key", key);
        }
        let resp: WhResponse = req.send().await?.error_for_status()?.json().await?;
        let photo = resp.data.into_iter().next()
            .ok_or_else(|| AppError::NotFound)?;
        let ext = match photo.file_type.as_str() {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            _ => "jpg",
        }.to_string();
        Ok(FetchedImage {
            source: SourceKind::Wallhaven,
            source_id: photo.id,
            photographer: None,
            source_url: Some(photo.url),
            image_url: Some(photo.path),
            local_path: None,
            download_location: None,
            width: Some(photo.dimension_x),
            height: Some(photo.dimension_y),
            ext,
        })
    }
}
