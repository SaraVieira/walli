use std::path::PathBuf;

#[derive(Clone)]
pub struct Cache { pub dir: PathBuf }
impl Cache {
    pub fn new(dir: PathBuf) -> std::io::Result<Self> { std::fs::create_dir_all(&dir)?; Ok(Self { dir }) }
}
