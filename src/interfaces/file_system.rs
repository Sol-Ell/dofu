use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::models::FsEntries;

pub trait FileSystemRepository: 'static + Sized + Send + Sync {
    fn get_base_path(&self) -> Arc<Path>;

    fn set_base_path(&mut self, value: PathBuf);

    fn get_folder_entries(
        &self,
        path: &Path,
    ) -> impl Future<Output = Option<FsEntries>> + Send + Sync;
}
