use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub parent_id: i64,
    pub description: String,
    pub s3_key: String,
    pub file_size_bytes: i64,
    pub uploaded_at: NaiveDateTime,
}
