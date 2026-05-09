use crate::cache::Cache;
use crate::db::{queries, Pool};
use crate::errors::{AppError, AppResult};
use crate::sources::{
    bing::Bing, http::HTTP, pool::Pool as SrcPool, unsplash::Unsplash, FetchContext, FetchedImage,
    SourceKind, WallpaperSource,
};
use crate::wallpaper_setter;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Listener, Manager};
use tokio::sync::mpsc;
use tokio::time::{sleep_until, Instant};

#[derive(Debug)]
#[allow(dead_code)]
pub enum SchedulerMsg {
    NextNow,
    Reschedule,
    Wake,
    Shutdown,
}

#[derive(Clone)]
pub struct SchedulerHandle {
    pub tx: mpsc::Sender<SchedulerMsg>,
}

pub async fn start(app: AppHandle) -> anyhow::Result<SchedulerHandle> {
    let (tx, rx) = mpsc::channel::<SchedulerMsg>(16);
    let app2 = app.clone();
    tokio::spawn(async move {
        if let Err(e) = run(app2, rx).await {
            tracing::error!(?e, "scheduler exited");
        }
    });
    let handle = SchedulerHandle { tx: tx.clone() };
    // bridge wake event into scheduler
    let tx_wake = tx.clone();
    let app_listen = app.clone();
    app_listen.listen("wake", move |_| {
        let tx_wake = tx_wake.clone();
        tokio::spawn(async move {
            let _ = tx_wake.send(SchedulerMsg::Wake).await;
        });
    });
    Ok(handle)
}

async fn run(app: AppHandle, mut rx: mpsc::Receiver<SchedulerMsg>) -> AppResult<()> {
    let pool: Arc<Pool> = Arc::new(app.state::<Pool>().inner().clone());
    let cache: Arc<Cache> = Arc::new(app.state::<Cache>().inner().clone());
    let src_pool = SrcPool::new();

    loop {
        let settings = queries::get_settings(&pool).await?;
        let paused = settings.get("paused").map(String::as_str) == Some("true");
        let interval = settings
            .get("interval_seconds")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(3600);
        let last = settings
            .get("last_rotation_at")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        let now = Utc::now().timestamp();

        let next_due_secs = if paused {
            i64::MAX
        } else {
            last.saturating_add(interval as i64)
        };
        let wait = if next_due_secs <= now {
            Duration::from_secs(0)
        } else {
            Duration::from_secs((next_due_secs - now) as u64)
        };

        let deadline = Instant::now() + wait;
        tracing::info!(?paused, interval_seconds = interval, "scheduler tick computed: wait {:?}", wait);
        tokio::select! {
            _ = sleep_until(deadline) => {
                if !paused { run_rotation(&app, &pool, &cache, &src_pool).await.ok(); }
            }
            msg = rx.recv() => {
                match msg {
                    Some(SchedulerMsg::NextNow) => { run_rotation(&app, &pool, &cache, &src_pool).await.ok(); }
                    Some(SchedulerMsg::Reschedule) => {} // loop top recomputes
                    Some(SchedulerMsg::Wake) => {
                        tracing::info!("wake event received");
                        // if overdue, fire immediately
                        let s = queries::get_settings(&pool).await?;
                        let last2 = s.get("last_rotation_at").and_then(|x| x.parse::<i64>().ok()).unwrap_or(0);
                        let int2 = s.get("interval_seconds").and_then(|x| x.parse::<u64>().ok()).unwrap_or(3600);
                        let is_overdue = Utc::now().timestamp() - last2 >= int2 as i64;
                        if is_overdue {
                            run_rotation(&app, &pool, &cache, &src_pool).await.ok();
                        }
                        tracing::info!(overdue = is_overdue, "wake handling complete");
                    }
                    Some(SchedulerMsg::Shutdown) | None => break,
                }
            }
        }
    }
    Ok(())
}

async fn run_rotation(
    app: &AppHandle,
    pool: &Pool,
    cache: &Cache,
    src_pool: &SrcPool,
) -> AppResult<()> {
    tracing::info!("rotation start");
    rotate_one_display(app, pool, cache, src_pool).await?;
    queries::set_setting(
        pool,
        "last_rotation_at",
        &Utc::now().timestamp().to_string(),
    )
    .await?;
    Ok(())
}

async fn rotate_one_display(
    app: &AppHandle,
    pool: &Pool,
    cache: &Cache,
    src_pool: &SrcPool,
) -> AppResult<()> {
    let s = queries::get_settings(pool).await?;
    let mut candidates = Vec::new();
    if s.get("source_unsplash_enabled").map(String::as_str) == Some("true")
        && s.get("unsplash_api_key")
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    {
        candidates.push(SourceKind::Unsplash);
    }
    let today = Utc::now().format("%Y-%m-%d").to_string();
    if s.get("source_bing_enabled").map(String::as_str) == Some("true")
        && s.get("last_bing_fetch_date").map(String::as_str) != Some(today.as_str())
    {
        candidates.push(SourceKind::Bing);
    }

    tracing::info!(candidates = ?candidates, "eligible sources");

    let kind = match src_pool.pick(&candidates) {
        Ok(k) => k,
        Err(_) => {
            tracing::warn!("no eligible sources, falling back to history");
            if let Some(w) = queries::random_history(pool).await? {
                apply_existing_wallpaper(app, pool, &w).await?;
                return Ok(());
            }
            tracing::error!("no fallback wallpaper available");
            let msg = "No sources eligible and no fallback available";
            tracing::error!(message = %msg, "scheduler error");
            let _ = app.emit("error", msg);
            return Ok(());
        }
    };
    tracing::info!(?kind, "picked source");

    let active_id = s
        .get("active_collection_id")
        .and_then(|x| x.parse::<i64>().ok());
    let tags: Vec<String> = if let Some(id) = active_id {
        queries::list_collections(pool)
            .await?
            .into_iter()
            .find(|c| c.id == id)
            .map(|c| c.tags)
            .unwrap_or_default()
    } else {
        vec![]
    };

    let mut api_keys = HashMap::new();
    if let Some(k) = s.get("unsplash_api_key") {
        if !k.is_empty() {
            api_keys.insert(SourceKind::Unsplash, k.clone());
        }
    }

    let ctx = FetchContext {
        tags,
        api_keys,
        today: today.clone(),
    };

    let source: Box<dyn WallpaperSource> = match kind {
        SourceKind::Unsplash => Box::new(Unsplash),
        SourceKind::Bing => Box::new(Bing),
    };

    let fetched = match fetch_with_retry(source.as_ref(), &ctx).await {
        Ok(f) => f,
        Err(e) => {
            tracing::warn!(?kind, ?e, "source failed, cooling down 5min");
            src_pool.cooldown(kind, Duration::from_secs(300));
            let msg = format!("{kind:?} failed: {e}");
            tracing::error!(message = %msg, "scheduler error");
            let _ = app.emit("error", &msg);
            return Ok(());
        }
    };
    tracing::info!(source_id = %fetched.source_id, photographer = ?fetched.photographer, "source fetched");

    let url = fetched
        .image_url
        .clone()
        .ok_or_else(|| AppError::Invalid("no image url".into()))?;
    let bytes = HTTP
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let p = cache.write(&bytes, &fetched.ext)?;
    let file_path = p.to_string_lossy().to_string();

    let w = queries::Wallpaper {
        id: 0,
        source: fetched.source.as_str().into(),
        source_id: fetched.source_id.clone(),
        photographer: fetched.photographer.clone(),
        title: fetched.title.clone(),
        source_url: fetched.source_url.clone(),
        file_path: file_path.clone(),
        is_local: false,
        width: fetched.width,
        height: fetched.height,
        fetched_at: Utc::now().timestamp(),
    };
    let wid = queries::upsert_wallpaper(pool, &w).await?;

    // Unsplash compliance ping (non-fatal)
    if let Some(dl) = &fetched.download_location {
        if let Some(key) = ctx_key_for(&ctx, SourceKind::Unsplash) {
            match HTTP
                .get(dl)
                .header("Authorization", format!("Client-ID {key}"))
                .send()
                .await
            {
                Ok(_) => tracing::debug!("unsplash compliance ping sent"),
                Err(e) => tracing::warn!(?e, "unsplash compliance ping failed"),
            }
            queries::mark_download_tracked(pool, wid).await.ok();
        }
    }

    tracing::info!(file_path = %file_path, "applying wallpaper");
    apply_and_record(app, pool, wid, &file_path).await?;
    tracing::info!("wallpaper applied");

    if kind == SourceKind::Bing {
        queries::set_setting(pool, "last_bing_fetch_date", &today).await?;
    }
    Ok(())
}

fn ctx_key_for(ctx: &FetchContext, kind: SourceKind) -> Option<&str> {
    ctx.api_keys.get(&kind).map(|s| s.as_str())
}

async fn apply_and_record(
    app: &AppHandle,
    pool: &Pool,
    wallpaper_id: i64,
    file_path: &str,
) -> AppResult<()> {
    wallpaper_setter::set_all_on_main(app, std::path::Path::new(file_path)).await?;
    queries::record_history(pool, wallpaper_id, Utc::now().timestamp()).await?;
    let w = queries::get_wallpaper(pool, wallpaper_id).await?;
    if let Some(w) = w {
        let _ = app.emit("wallpaper-changed", &w);
    }
    Ok(())
}

async fn apply_existing_wallpaper(
    app: &AppHandle,
    pool: &Pool,
    w: &queries::Wallpaper,
) -> AppResult<()> {
    apply_and_record(app, pool, w.id, &w.file_path).await
}

async fn fetch_with_retry(
    src: &dyn WallpaperSource,
    ctx: &FetchContext,
) -> AppResult<FetchedImage> {
    let mut delay = Duration::from_secs(1);
    for attempt in 0..3 {
        match src.fetch(ctx).await {
            Ok(v) => return Ok(v),
            Err(e) if attempt < 2 && e.is_retryable() => {
                tracing::warn!(attempt, ?e, "retry");
                tokio::time::sleep(delay).await;
                delay *= 4;
            }
            Err(e) => return Err(e),
        }
    }
    Err(AppError::Internal("retries exhausted".into()))
}
