use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod migrations;
pub mod queries;

#[cfg(test)]
mod tests;

pub type Pool = Arc<Mutex<Connection>>;

pub async fn init(path: &Path) -> anyhow::Result<Pool> {
    let mut conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrations::run(&mut conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}
