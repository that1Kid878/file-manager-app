use std::error::Error;

use file_manager_backend::api;
use file_manager_backend::core::state;
use file_manager_backend::infrastructure::database::create_pool;
use file_manager_backend::infrastructure::repos::files::SQLiteFileRepository;
use file_manager_backend::infrastructure::repos::folders::SQLiteFolderRepository;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = create_pool().await.unwrap();
    let file_repo = SQLiteFileRepository::new(pool.clone());
    let folder_repo = SQLiteFolderRepository::new(pool.clone());
    let state = state::create_state(file_repo, folder_repo);
    let app = api::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
