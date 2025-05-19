use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use scc::HashMap;
use tokio::sync::RwLock;

use crate::models::FsEntries;

use super::DummyFileSystem;

const DEFAULT_CACHE_LIFESPAN: Duration = Duration::new(60 * 60 * 24, 0);

#[derive(Debug, Clone)]
struct CachedFsEntries(FsEntries, Instant);

#[derive(Debug, Clone)]
pub struct CachedFileSystem {
    inner: DummyFileSystem,
    cache: Arc<HashMap<Box<Path>, CachedFsEntries, ahash::RandomState>>,
    cache_lifespan: Arc<RwLock<Duration>>,
}

impl CachedFileSystem {
    pub async fn from_base_path(path: PathBuf) -> Self {
        Self {
            inner: DummyFileSystem::from_base_path(path).await,
            cache: Arc::new(HashMap::with_hasher(ahash::RandomState::new())),
            cache_lifespan: Arc::new(RwLock::new(DEFAULT_CACHE_LIFESPAN)),
        }
    }
}

impl crate::interfaces::FileSystemRepository for CachedFileSystem {
    fn get_base_path(&self) -> Arc<Path> {
        self.inner.get_base_path()
    }

    fn set_base_path(&mut self, value: PathBuf) {
        self.inner.set_base_path(value);
    }

    fn get_folder_entries(
        &self,
        path: &Path,
    ) -> impl Future<Output = Option<FsEntries>> + Send + Sync {
        async move {
            if let Some(cached) = self.cache.get_async(path).await {
                let cache_lifespan = self.cache_lifespan.read().await.clone();
                let now = Instant::now();

                if (now - cached.1) > cache_lifespan {
                    return Some(cached.0.clone());
                }

                drop(cached);

                let Some(entries) = self.inner.get_folder_entries(path).await else {
                    return None;
                };

                let new_cache = CachedFsEntries(entries.clone(), now);

                self.cache
                    .update_async(path, |_, cached| *cached = new_cache)
                    .await;

                return Some(entries);
            }
            let now = Instant::now();
            let Some(entries) = self.inner.get_folder_entries(path).await else {
                return None;
            };

            self.cache
                .upsert_async(
                    path.to_path_buf().into_boxed_path(),
                    CachedFsEntries(entries.clone(), now),
                )
                .await;

            Some(entries)
        }
    }
}
