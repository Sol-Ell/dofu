use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use tracing::{debug, trace};

use crate::{
    interfaces::FileSystemRepository,
    models::{FsEntries, FsEntry},
    util::FS_ENTRY_LIMIT,
};

#[derive(Debug, Clone)]
pub struct DummyFileSystem {
    base_path: Arc<Path>,
}

impl DummyFileSystem {
    pub fn from_base_path(path: PathBuf) -> impl Future<Output = Self> + Send + Sync {
        async move {
            Self {
                base_path: Arc::from(path.into_boxed_path()),
            }
        }
    }
}

impl FileSystemRepository for DummyFileSystem {
    fn get_base_path(&self) -> Arc<Path> {
        self.base_path.clone()
    }

    fn set_base_path(&mut self, value: PathBuf) {
        self.base_path = Arc::from(value.into_boxed_path());
    }

    #[tracing::instrument(skip(self))]
    fn get_folder_entries(
        &self,
        path: &Path,
    ) -> impl Future<Output = std::option::Option<FsEntries>> + Send + Sync {
        async move {
            let mut absolute_path = self.base_path.to_path_buf();
            absolute_path.push(&path);

            let Some(files) = load_fs_entries(&absolute_path).await else {
                debug!("Failed to load file system entries from {absolute_path:?}");
                return None;
            };

            Some(files)
        }
    }
}

#[tracing::instrument]
async fn load_fs_entries(absolute_path: &Path) -> Option<FsEntries> {
    let mut iter = match tokio::fs::read_dir(absolute_path).await {
        Ok(t) => t,
        Err(e) => {
            trace!("Failed to open directory. Error {e:?}");
            return None;
        }
    };

    let mut entries = Vec::new();

    while FS_ENTRY_LIMIT.is_none() || Some(entries.len()) < FS_ENTRY_LIMIT {
        let entry = match iter.next_entry().await {
            Ok(Some(t)) => t,
            Ok(None) => break,
            Err(e) => {
                debug!("Stopping enumeration of filesystem entries. Error {e:?}");
                break;
            }
        };

        let metadata = match entry.metadata().await {
            Ok(t) => t,
            Err(e) => {
                debug!("Skipping filesystem entry {:?}. Error {e:?}", entry.path());
                continue;
            }
        };

        let name = match entry.file_name().into_string() {
            Ok(t) => t,
            Err(_) => {
                debug!(
                    "Skipping filesystem entry {:?}. Invalid UTF-8 name.",
                    entry.path()
                );
                continue;
            }
        };

        entries.push(FsEntry::from_metadata(name, metadata));
    }

    if FS_ENTRY_LIMIT.is_some() && Some(entries.len()) == FS_ENTRY_LIMIT {
        debug!("Reached the limit of entries in the folder, stopping.");
    }

    Some(FsEntries::from(entries))
}
