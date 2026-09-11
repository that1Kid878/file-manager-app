use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct CreateFileDTO {
    pub name: String,
    pub parent_id: Option<i64>,
    pub description: Option<i64>,
    pub s3_key: String,
    pub file_size_bytes: i64,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct CreateFolderDTO {
    pub name: String,
    pub parent_id: Option<i64>,
}
