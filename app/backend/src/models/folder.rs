use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Folder {
    id: i32,
    name: String,
    parent_id: i32,
    created_at: DateTime<Utc>,
}
