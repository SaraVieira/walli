use crate::db::Pool;
use crate::errors::AppResult;
use rusqlite::{params, OptionalExtension};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Wallpaper {
    pub id: i64,
    pub source: String,
    pub source_id: String,
    pub photographer: Option<String>,
    pub title: Option<String>,
    pub source_url: Option<String>,
    pub file_path: String,
    pub is_local: bool,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub fetched_at: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HistoryEntry {
    pub history_id: i64,
    pub wallpaper: Wallpaper,
    pub set_at: i64,
    pub display_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub tags: Vec<String>,
}

pub fn upsert_wallpaper(pool: &Pool, w: &Wallpaper) -> AppResult<i64> {
    let conn = pool.lock().unwrap();
    conn.execute(
        "INSERT INTO wallpapers (source, source_id, photographer, title, source_url, file_path, is_local, width, height, fetched_at)
         VALUES (?,?,?,?,?,?,?,?,?,?)
         ON CONFLICT(source, source_id) DO UPDATE SET file_path = excluded.file_path",
        params![w.source, w.source_id, w.photographer, w.title, w.source_url, w.file_path,
                w.is_local as i64, w.width, w.height, w.fetched_at],
    )?;
    let id: i64 = conn.query_row(
        "SELECT id FROM wallpapers WHERE source = ? AND source_id = ?",
        params![w.source, w.source_id],
        |r| r.get(0),
    )?;
    Ok(id)
}

pub fn mark_download_tracked(pool: &Pool, wallpaper_id: i64) -> AppResult<()> {
    pool.lock().unwrap().execute(
        "UPDATE wallpapers SET download_tracked = 1 WHERE id = ?",
        params![wallpaper_id],
    )?;
    Ok(())
}

pub fn record_history(
    pool: &Pool,
    wallpaper_id: i64,
    set_at: i64,
    display_id: Option<&str>,
) -> AppResult<i64> {
    let conn = pool.lock().unwrap();
    conn.execute(
        "INSERT INTO history (wallpaper_id, set_at, display_id) VALUES (?,?,?)",
        params![wallpaper_id, set_at, display_id],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_history(pool: &Pool, limit: u32, offset: u32) -> AppResult<Vec<HistoryEntry>> {
    let conn = pool.lock().unwrap();
    let sql = "SELECT h.id, h.set_at, h.display_id, w.id, w.source, w.source_id, w.photographer, w.title, w.source_url,
                w.file_path, w.is_local, w.width, w.height, w.fetched_at
         FROM history h JOIN wallpapers w ON w.id = h.wallpaper_id
         ORDER BY h.set_at DESC LIMIT ? OFFSET ?";
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt
        .query_map(params![limit, offset], |r| {
            Ok(HistoryEntry {
                history_id: r.get(0)?,
                set_at: r.get(1)?,
                display_id: r.get(2)?,
                wallpaper: Wallpaper {
                    id: r.get(3)?,
                    source: r.get(4)?,
                    source_id: r.get(5)?,
                    photographer: r.get(6)?,
                    title: r.get(7)?,
                    source_url: r.get(8)?,
                    file_path: r.get(9)?,
                    is_local: r.get::<_, i64>(10)? != 0,
                    width: r.get(11)?,
                    height: r.get(12)?,
                    fetched_at: r.get(13)?,
                },
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn get_wallpaper(pool: &Pool, id: i64) -> AppResult<Option<Wallpaper>> {
    let conn = pool.lock().unwrap();
    Ok(conn.query_row(
        "SELECT w.id, w.source, w.source_id, w.photographer, w.title, w.source_url, w.file_path, w.is_local,
                w.width, w.height, w.fetched_at
         FROM wallpapers w WHERE w.id = ?",
        params![id],
        |r| Ok(Wallpaper {
            id: r.get(0)?, source: r.get(1)?, source_id: r.get(2)?,
            photographer: r.get(3)?, title: r.get(4)?, source_url: r.get(5)?, file_path: r.get(6)?,
            is_local: r.get::<_, i64>(7)? != 0,
            width: r.get(8)?, height: r.get(9)?, fetched_at: r.get(10)?,
        })).optional()?)
}

pub fn random_history(pool: &Pool) -> AppResult<Option<Wallpaper>> {
    let conn = pool.lock().unwrap();
    Ok(conn.query_row(
        "SELECT w.id, w.source, w.source_id, w.photographer, w.title, w.source_url, w.file_path, w.is_local,
                w.width, w.height, w.fetched_at
         FROM wallpapers w
         WHERE w.file_path IS NOT NULL
         ORDER BY RANDOM() LIMIT 1",
        [],
        |r| Ok(Wallpaper {
            id: r.get(0)?, source: r.get(1)?, source_id: r.get(2)?,
            photographer: r.get(3)?, title: r.get(4)?, source_url: r.get(5)?, file_path: r.get(6)?,
            is_local: r.get::<_, i64>(7)? != 0,
            width: r.get(8)?, height: r.get(9)?, fetched_at: r.get(10)?,
        })).optional()?)
}

pub fn list_collections(pool: &Pool) -> AppResult<Vec<Collection>> {
    let conn = pool.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name FROM collections ORDER BY name")?;
    let cols: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut out = Vec::new();
    for (id, name) in cols {
        let mut tag_stmt =
            conn.prepare("SELECT tag FROM collection_tags WHERE collection_id = ? ORDER BY tag")?;
        let tags: Vec<String> = tag_stmt
            .query_map(params![id], |r| r.get::<_, String>(0))?
            .collect::<rusqlite::Result<_>>()?;
        out.push(Collection { id, name, tags });
    }
    Ok(out)
}

pub fn create_collection(
    pool: &Pool,
    name: &str,
    tags: &[String],
    now: i64,
) -> AppResult<Collection> {
    let mut conn = pool.lock().unwrap();
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO collections (name, created_at) VALUES (?, ?)",
        params![name, now],
    )?;
    let id = tx.last_insert_rowid();
    for tag in tags {
        tx.execute(
            "INSERT INTO collection_tags (collection_id, tag) VALUES (?, ?)",
            params![id, tag],
        )?;
    }
    tx.commit()?;
    Ok(Collection {
        id,
        name: name.to_string(),
        tags: tags.to_vec(),
    })
}

pub fn update_collection(
    pool: &Pool,
    id: i64,
    name: &str,
    tags: &[String],
) -> AppResult<Collection> {
    let mut conn = pool.lock().unwrap();
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE collections SET name = ? WHERE id = ?",
        params![name, id],
    )?;
    tx.execute(
        "DELETE FROM collection_tags WHERE collection_id = ?",
        params![id],
    )?;
    for tag in tags {
        tx.execute(
            "INSERT INTO collection_tags (collection_id, tag) VALUES (?, ?)",
            params![id, tag],
        )?;
    }
    tx.commit()?;
    Ok(Collection {
        id,
        name: name.to_string(),
        tags: tags.to_vec(),
    })
}

pub fn delete_collection(pool: &Pool, id: i64) -> AppResult<()> {
    pool.lock()
        .unwrap()
        .execute("DELETE FROM collections WHERE id = ?", params![id])?;
    Ok(())
}

pub fn get_settings(pool: &Pool) -> AppResult<HashMap<String, String>> {
    let conn = pool.lock().unwrap();
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
    Ok(rows.collect::<rusqlite::Result<HashMap<_, _>>>()?)
}

pub fn set_setting(pool: &Pool, key: &str, value: &str) -> AppResult<()> {
    pool.lock().unwrap().execute(
        "INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value])?;
    Ok(())
}
