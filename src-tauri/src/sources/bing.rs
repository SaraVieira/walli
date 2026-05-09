use super::{http::HTTP, FetchContext, FetchedImage, SourceKind, WallpaperSource};
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use serde::Deserialize;

pub struct Bing;

#[derive(Deserialize)]
struct BingResp {
    images: Vec<BingImg>,
}
#[derive(Deserialize)]
struct BingImg {
    url: String,
    copyright: Option<String>,
    copyrightlink: Option<String>,
    startdate: String,
    title: Option<String>,
}

#[async_trait]
impl WallpaperSource for Bing {
    fn kind(&self) -> SourceKind {
        SourceKind::Bing
    }

    async fn fetch(&self, _ctx: &FetchContext) -> AppResult<FetchedImage> {
        tracing::debug!("bing fetching daily image");
        let resp: BingResp = HTTP
            .get("https://www.bing.com/HPImageArchive.aspx")
            .query(&[("format", "js"), ("idx", "0"), ("n", "1"), ("mkt", "en-US")])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let img = resp.images.into_iter().next().ok_or(AppError::NotFound)?;
        let uhd_url = format!("https://www.bing.com{}", img.url.replace("_1920x1080", "_UHD"));
        let fallback_url = format!("https://www.bing.com{}", img.url);
        let full_url = match HTTP.head(&uhd_url).send().await {
            Ok(r) if r.status().is_success() => uhd_url,
            _ => {
                tracing::debug!("bing UHD url unavailable, falling back to 1920x1080");
                fallback_url
            }
        };
        Ok(FetchedImage {
            source: SourceKind::Bing,
            source_id: format!("{}-en-US", img.startdate),
            photographer: img.copyright,
            title: img.title,
            source_url: img.copyrightlink,
            image_url: Some(full_url),
            download_location: None,
            width: None,
            height: None,
            ext: "jpg".into(),
        })
    }
}
