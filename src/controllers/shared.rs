use std::path::PathBuf;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{AppState, interfaces::FileSystemRepository, models::FsEntries};

#[derive(Debug, Deserialize)]
pub struct Request {
    path: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    folder_content: FsEntries,
}

// TODO: Cache json string of files within folder too.
#[tracing::instrument(skip(files_repository))]
pub async fn get_folder_content<T: FileSystemRepository>(
    State(AppState {
        files_repository, ..
    }): State<AppState<T>>,
    Query(query): Query<Request>,
) -> impl IntoResponse {
    let path = PathBuf::from(query.path);

    debug!("Providing fs entries of {path:?}");

    if let Some(entries) = files_repository.get_folder_entries(&path).await {
        debug!("Found {:?}", entries);
        let a = Response {
            folder_content: entries,
        };
        return Ok(Json(a));
    }

    return Err(format!("Invalid file path."));
}
