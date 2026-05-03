use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub type Pool = Arc<Mutex<Connection>>;

pub async fn init(path: &Path) -> anyhow::Result<Pool> {
    let conn = Connection::open(path)?;
    Ok(Arc::new(Mutex::new(conn)))
}
