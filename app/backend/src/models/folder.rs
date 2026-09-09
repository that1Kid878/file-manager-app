use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct FolderDbModel {
    id: i64,
    name: String,
    parent_id: i64,
    created_at: DateTime<Utc>,
}
