use rusqlite::Connection;

pub const MIGRATIONS: &[&str] = &[
    // 0001 — initial schema
    r#"
    CREATE TABLE collections (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        created_at INTEGER NOT NULL
    );
    CREATE TABLE collection_tags (
        collection_id INTEGER NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
        tag TEXT NOT NULL,
        PRIMARY KEY (collection_id, tag)
    );
    CREATE TABLE wallpapers (
        id INTEGER PRIMARY KEY,
        source TEXT NOT NULL,
        source_id TEXT NOT NULL,
        photographer TEXT,
        source_url TEXT,
        file_path TEXT NOT NULL,
        is_local INTEGER NOT NULL DEFAULT 0,
        download_tracked INTEGER NOT NULL DEFAULT 0,
        width INTEGER,
        height INTEGER,
        fetched_at INTEGER NOT NULL,
        UNIQUE (source, source_id)
    );
    CREATE TABLE history (
        id INTEGER PRIMARY KEY,
        wallpaper_id INTEGER NOT NULL REFERENCES wallpapers(id),
        set_at INTEGER NOT NULL,
        display_id TEXT
    );
    CREATE INDEX idx_history_set_at ON history(set_at DESC);
    CREATE TABLE favorites (
        wallpaper_id INTEGER PRIMARY KEY REFERENCES wallpapers(id),
        favorited_at INTEGER NOT NULL
    );
    CREATE TABLE settings (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );
    INSERT INTO settings (key, value) VALUES
        ('interval_seconds', '3600'),
        ('paused', 'false'),
        ('source_unsplash_enabled', 'true'),
        ('source_wallhaven_enabled', 'true'),
        ('source_bing_enabled', 'true'),
        ('source_apod_enabled', 'true'),
        ('source_local_enabled', 'false');
    "#,
    // 0002 — drop wallhaven, seed unsplash_api_key (idempotent on existing DBs)
    r#"
    DELETE FROM settings WHERE key = 'source_wallhaven_enabled';
    INSERT OR IGNORE INTO settings (key, value) VALUES ('unsplash_api_key', '');
    "#,
    // 0003 — drop NASA APOD and Local source settings
    r#"
    DELETE FROM settings WHERE key IN (
        'source_apod_enabled',
        'source_local_enabled',
        'local_folder_path',
        'last_apod_fetch_date'
    );
    "#,
    // 0004 — add title column to wallpapers
    r#"
    ALTER TABLE wallpapers ADD COLUMN title TEXT;
    "#,
    // 0005 — seed default Unsplash collections (Featured, Wallpapers, Space).
    // Each collection is added only if no collection with that name already exists,
    // so users who've created their own won't be overwritten.
    r#"
    INSERT INTO collections (name, created_at)
    SELECT 'Featured', strftime('%s', 'now')
    WHERE NOT EXISTS (SELECT 1 FROM collections WHERE name = 'Featured');

    INSERT INTO collections (name, created_at)
    SELECT 'Wallpapers', strftime('%s', 'now')
    WHERE NOT EXISTS (SELECT 1 FROM collections WHERE name = 'Wallpapers');

    INSERT INTO collections (name, created_at)
    SELECT 'Space', strftime('%s', 'now')
    WHERE NOT EXISTS (SELECT 1 FROM collections WHERE name = 'Space');

    INSERT OR IGNORE INTO collection_tags (collection_id, tag)
    SELECT id, 'featured' FROM collections WHERE name = 'Featured';

    INSERT OR IGNORE INTO collection_tags (collection_id, tag)
    SELECT id, 'wallpapers' FROM collections WHERE name = 'Wallpapers';

    INSERT OR IGNORE INTO collection_tags (collection_id, tag)
    SELECT id, 'space' FROM collections WHERE name = 'Space';

    INSERT OR IGNORE INTO settings (key, value)
    SELECT 'active_collection_id', CAST(id AS TEXT)
    FROM collections WHERE name = 'Featured';
    "#,
    // 0006 — drop favorites feature
    r#"
    DROP TABLE IF EXISTS favorites;
    "#,
];

pub fn run(conn: &mut Connection) -> rusqlite::Result<()> {
    conn.execute_batch("CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL)")?;
    let current: i64 = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_version",
        [],
        |r| r.get(0),
    )?;
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let v = (i as i64) + 1;
        if v > current {
            let tx = conn.transaction()?;
            tx.execute_batch(sql)?;
            tx.execute("INSERT INTO schema_version (version) VALUES (?)", [v])?;
            tx.commit()?;
        }
    }
    Ok(())
}
