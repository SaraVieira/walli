use async_trait::async_trait;
use serde::Deserialize;
use crate::errors::{AppError, AppResult};
use super::{http::HTTP, FetchContext, FetchedImage, SourceKind, WallpaperSource};

pub struct Apod;

#[derive(Deserialize)]
struct ApodResp {
    date: String,
    media_type: String,
    hdurl: Option<String>,
    url: String,
    copyright: Option<String>,
}

#[async_trait]
impl WallpaperSource for Apod {
    fn kind(&self) -> SourceKind { SourceKind::Apod }

    async fn fetch(&self, _ctx: &FetchContext) -> AppResult<FetchedImage> {
        let r: ApodResp = HTTP.get("https://api.nasa.gov/planetary/apod")
            .query(&[("api_key", "DEMO_KEY")])
            .send().await?.error_for_status()?.json().await?;
        if r.media_type != "image" {
            return Err(AppError::Invalid("APOD is not an image today".into()));
        }
        let url = r.hdurl.unwrap_or(r.url.clone());
        let ext = url.rsplit('.').next().unwrap_or("jpg").to_lowercase();
        let ext = if ["jpg","jpeg","png"].contains(&ext.as_str()) { ext } else { "jpg".into() };
        Ok(FetchedImage {
            source: SourceKind::Apod,
            source_id: r.date,
            photographer: r.copyright,
            source_url: Some(format!("https://apod.nasa.gov/apod/")),
            image_url: Some(url),
            local_path: None,
            download_location: None,
            width: None, height: None,
            ext,
        })
    }
}
