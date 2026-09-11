use std::sync::Arc;

use crate::infrastructure::repos::{files::SQLiteFileRepository, folders::SQLiteFolderRepository};

pub struct AppState {
    file_repo: SQLiteFileRepository,
    folder_repo: SQLiteFolderRepository,
}

pub fn create_state(
    file_repo: SQLiteFileRepository,
    folder_repo: SQLiteFolderRepository,
) -> Arc<AppState> {
    return Arc::new(AppState {
        file_repo: file_repo,
        folder_repo: folder_repo,
    });
}
