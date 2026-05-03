use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use crate::errors::{AppError, AppResult};
use super::SourceKind;

#[derive(Default)]
pub struct CooldownState {
    pub until: HashMap<SourceKind, Instant>,
}

#[derive(Clone, Default)]
pub struct Pool {
    state: Arc<Mutex<CooldownState>>,
}

impl Pool {
    pub fn new() -> Self { Self::default() }

    pub fn cooldown(&self, kind: SourceKind, dur: Duration) {
        let mut s = self.state.lock().unwrap();
        s.until.insert(kind, Instant::now() + dur);
    }

    pub fn eligible(&self, candidates: &[SourceKind]) -> Vec<SourceKind> {
        let s = self.state.lock().unwrap();
        let now = Instant::now();
        candidates.iter().copied().filter(|k| {
            s.until.get(k).map(|t| *t <= now).unwrap_or(true)
        }).collect()
    }

    pub fn pick(&self, candidates: &[SourceKind]) -> AppResult<SourceKind> {
        let elig = self.eligible(candidates);
        elig.choose(&mut rand::thread_rng()).copied()
            .ok_or_else(|| AppError::Invalid("No eligible sources".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_excludes_source() {
        let p = Pool::new();
        p.cooldown(SourceKind::Unsplash, Duration::from_secs(60));
        let elig = p.eligible(&[SourceKind::Unsplash, SourceKind::Bing]);
        assert_eq!(elig, vec![SourceKind::Bing]);
    }

    #[test]
    fn pick_errors_when_empty() {
        let p = Pool::new();
        assert!(p.pick(&[]).is_err());
    }
}
