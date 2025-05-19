use std::path::PathBuf;

use crate::{
    interfaces::FileSystemRepository, repositories::CachedFileSystem, util::UPLOADS_FOLDER_PATH,
};

#[derive(Debug, Clone)]
pub struct AppState<T: FileSystemRepository> {
    pub files_repository: T,
}

impl AppState<CachedFileSystem> {
    pub async fn default() -> Self {
        Self {
            files_repository: CachedFileSystem::from_base_path(PathBuf::from(UPLOADS_FOLDER_PATH))
                .await,
        }
    }
}
