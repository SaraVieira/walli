use async_trait::async_trait;
use serde::Deserialize;
use crate::errors::{AppError, AppResult};
use super::{http::HTTP, FetchContext, FetchedImage, SourceKind, WallpaperSource};

pub struct Bing;

#[derive(Deserialize)]
struct BingResp { images: Vec<BingImg> }
#[derive(Deserialize)]
struct BingImg {
    url: String,
    copyright: Option<String>,
    copyrightlink: Option<String>,
    startdate: String,
}

#[async_trait]
impl WallpaperSource for Bing {
    fn kind(&self) -> SourceKind { SourceKind::Bing }

    async fn fetch(&self, _ctx: &FetchContext) -> AppResult<FetchedImage> {
        let resp: BingResp = HTTP.get("https://www.bing.com/HPImageArchive.aspx")
            .query(&[("format", "js"), ("idx", "0"), ("n", "1"), ("mkt", "en-US")])
            .send().await?.error_for_status()?.json().await?;
        let img = resp.images.into_iter().next().ok_or(AppError::NotFound)?;
        let path = img.url.replace("_1920x1080", "_UHD");
        let full_url = format!("https://www.bing.com{path}");
        Ok(FetchedImage {
            source: SourceKind::Bing,
            source_id: format!("{}-en-US", img.startdate),
            photographer: img.copyright,
            source_url: img.copyrightlink,
            image_url: Some(full_url),
            local_path: None,
            download_location: None,
            width: None, height: None,
            ext: "jpg".into(),
        })
    }
}
