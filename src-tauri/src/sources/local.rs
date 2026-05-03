use async_trait::async_trait;
use rand::seq::SliceRandom;
use std::path::PathBuf;
use crate::errors::{AppError, AppResult};
use super::{FetchContext, FetchedImage, SourceKind, WallpaperSource};

pub struct Local;

const EXTS: &[&str] = &["jpg", "jpeg", "png", "heic", "webp"];

fn scan(root: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() { stack.push(p); continue; }
            if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                if EXTS.contains(&ext.to_lowercase().as_str()) { out.push(p); }
            }
        }
    }
    out
}

#[async_trait]
impl WallpaperSource for Local {
    fn kind(&self) -> SourceKind { SourceKind::Local }

    async fn fetch(&self, ctx: &FetchContext) -> AppResult<FetchedImage> {
        let folder = ctx.local_folder.as_deref()
            .ok_or_else(|| AppError::Invalid("Local folder not set".into()))?;
        let root = std::path::Path::new(folder);
        if !root.exists() { return Err(AppError::Invalid("Local folder does not exist".into())); }
        let files = scan(root);
        let pick = files.choose(&mut rand::thread_rng())
            .ok_or_else(|| AppError::Invalid("No images in local folder".into()))?;
        let ext = pick.extension().and_then(|s| s.to_str()).unwrap_or("jpg").to_lowercase();
        let path_str = pick.to_string_lossy().to_string();
        Ok(FetchedImage {
            source: SourceKind::Local,
            source_id: path_str.clone(),
            photographer: None,
            source_url: None,
            image_url: None,
            local_path: Some(path_str),
            download_location: None,
            width: None, height: None,
            ext,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[tokio::test]
    async fn picks_image_from_folder() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("a.jpg"), b"x").unwrap();
        std::fs::write(dir.path().join("b.txt"), b"x").unwrap();
        let ctx = FetchContext {
            tags: vec![],
            api_keys: HashMap::new(),
            local_folder: Some(dir.path().to_string_lossy().into()),
            today: "2026-01-01".into(),
        };
        let img = Local.fetch(&ctx).await.unwrap();
        assert_eq!(img.source, SourceKind::Local);
        assert!(img.local_path.unwrap().ends_with("a.jpg"));
    }

    #[tokio::test]
    async fn errors_when_folder_empty() {
        let dir = TempDir::new().unwrap();
        let ctx = FetchContext {
            tags: vec![], api_keys: HashMap::new(),
            local_folder: Some(dir.path().to_string_lossy().into()),
            today: "2026-01-01".into(),
        };
        assert!(Local.fetch(&ctx).await.is_err());
    }
}
