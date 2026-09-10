use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct File {
    id: i32,
    name: String,
    parent_id: i32,
    description: String,
    s3_key: String,
    file_size_bytes: i32,
    uploaded_at: NaiveDateTime,
}
