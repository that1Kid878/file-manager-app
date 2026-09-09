use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct File {
    id: i64,
    name: String,
    parent_id: i64,
    description: String,
    s3_key: String,
    file_size_bytes: i64,
    uploaded_at: DateTime<Utc>,
}
