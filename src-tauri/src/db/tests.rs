use super::*;
use crate::db::queries;
use tempfile::TempDir;

async fn fresh_pool() -> (Pool, TempDir) {
    let dir = TempDir::new().unwrap();
    let pool = init(&dir.path().join("t.db")).await.unwrap();
    (pool, dir)
}

#[tokio::test]
async fn migrations_run_idempotently() {
    let (_pool, dir) = fresh_pool().await;
    let _pool2 = init(&dir.path().join("t.db")).await.unwrap();
}

#[tokio::test]
async fn upsert_wallpaper_and_history() {
    let (pool, _dir) = fresh_pool().await;
    let w = queries::Wallpaper {
        id: 0,
        source: "unsplash".into(),
        source_id: "abc".into(),
        photographer: Some("alice".into()),
        title: Some("Test photo".into()),
        source_url: None,
        file_path: "/tmp/a.jpg".into(),
        is_local: false,
        width: Some(1920),
        height: Some(1080),
        fetched_at: 1,
    };
    let id = queries::upsert_wallpaper(&pool, &w).await.unwrap();
    assert!(id > 0);
    let id2 = queries::upsert_wallpaper(&pool, &w).await.unwrap();
    assert_eq!(id, id2, "upsert should return same id");
    queries::record_history(&pool, id, 100, None).await.unwrap();
    let history = queries::list_history(&pool, 10, 0).await.unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].wallpaper.source_id, "abc");
}

#[tokio::test]
async fn collection_crud() {
    let (pool, _dir) = fresh_pool().await;
    let baseline = queries::list_collections(&pool).await.unwrap().len();
    let c = queries::create_collection(&pool, "Nature", &["mountains".into(), "forest".into()], 0)
        .await
        .unwrap();
    assert_eq!(c.tags.len(), 2);
    let updated = queries::update_collection(&pool, c.id, "Nature 2", &["ocean".into()])
        .await
        .unwrap();
    assert_eq!(updated.tags, vec!["ocean"]);
    assert_eq!(
        queries::list_collections(&pool).await.unwrap().len(),
        baseline + 1
    );
    queries::delete_collection(&pool, c.id).await.unwrap();
    assert_eq!(
        queries::list_collections(&pool).await.unwrap().len(),
        baseline
    );
}

#[tokio::test]
async fn default_collections_seeded() {
    let (pool, _dir) = fresh_pool().await;
    let names: Vec<String> = queries::list_collections(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.name)
        .collect();
    for expected in ["Featured", "Wallpapers", "Space"] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing default collection {expected}, got {names:?}"
        );
    }
}

#[tokio::test]
async fn settings_defaults_seed() {
    let (pool, _dir) = fresh_pool().await;
    let s = queries::get_settings(&pool).await.unwrap();
    assert_eq!(s.get("interval_seconds").map(String::as_str), Some("3600"));
    queries::set_setting(&pool, "interval_seconds", "7200")
        .await
        .unwrap();
    let s2 = queries::get_settings(&pool).await.unwrap();
    assert_eq!(s2.get("interval_seconds").map(String::as_str), Some("7200"));
}
