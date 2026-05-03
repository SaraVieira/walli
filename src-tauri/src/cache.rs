use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use crate::errors::AppResult;

#[derive(Clone)]
pub struct Cache { pub dir: PathBuf }

impl Cache {
    pub fn new(dir: PathBuf) -> std::io::Result<Self> {
        std::fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    pub fn path_for(&self, bytes: &[u8], ext: &str) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hex = hex::encode(hasher.finalize());
        let safe_ext = if ext.is_empty() { "jpg".into() } else { ext.trim_start_matches('.').to_lowercase() };
        self.dir.join(format!("{}.{}", hex, safe_ext))
    }

    pub fn write(&self, bytes: &[u8], ext: &str) -> AppResult<PathBuf> {
        let p = self.path_for(bytes, ext);
        if !p.exists() { std::fs::write(&p, bytes)?; }
        Ok(p)
    }

    pub fn exists(&self, p: &Path) -> bool { p.exists() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn write_is_content_addressed_and_idempotent() {
        let dir = TempDir::new().unwrap();
        let cache = Cache::new(dir.path().to_path_buf()).unwrap();
        let p1 = cache.write(b"hello", "jpg").unwrap();
        let p2 = cache.write(b"hello", "jpg").unwrap();
        assert_eq!(p1, p2);
        assert!(p1.to_str().unwrap().ends_with(".jpg"));
        let p3 = cache.write(b"different", "png").unwrap();
        assert_ne!(p1, p3);
    }
}
