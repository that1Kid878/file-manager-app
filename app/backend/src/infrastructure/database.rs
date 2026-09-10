use std::env;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::infrastructure::errors::InfrastructureError;

pub async fn create_pool() -> Result<SqlitePool, InfrastructureError> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(|e| InfrastructureError::MigrationError(e.to_string()))?;

    Ok(pool)
}
