use async_trait::async_trait;

use crate::{domains::errors::DomainError, models::file::File};

#[async_trait]
pub trait FileRepo: Send + Sync {
    async fn get_file_by_name(&self, name: &str) -> Result<File, DomainError>;
    async fn get_file_by_id(&self, id: i64) -> Result<File, DomainError>;
    async fn get_file_by_s3_key(&self, key: &str) -> Result<File, DomainError>;
    async fn create_file(&self, file: &File) -> Result<(), DomainError>;
    async fn update_file_name(&self, id: i64, new_name: &str) -> Result<File, DomainError>;
    async fn delete_file(&self, id: i64) -> Result<(), DomainError>;
}
