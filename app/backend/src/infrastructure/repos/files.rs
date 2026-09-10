use sqlx::SqlitePool;

use crate::{infrastructure::errors::InfrastructureError, models::file::File};

#[derive(Clone)]
pub struct SQLiteFileRepository {
    pool: SqlitePool,
}

impl SQLiteFileRepository {
    pub fn new(pool: SqlitePool) -> Self {
        return Self { pool };
    }

    pub async fn get_all(&self, name: &str) -> Result<Vec<File>, InfrastructureError> {
        let files = sqlx::query_as::<_, File>("SELECT * FROM files")
            .bind(name)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(files)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<File>, InfrastructureError> {
        let file = sqlx::query_as::<_, File>("SELECT * FROM files WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(file)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<File>, InfrastructureError> {
        let file = sqlx::query_as::<_, File>("SELECT * FROM files WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(file)
    }

    pub async fn get_by_s3_key(&self, s3_key: &str) -> Result<Option<File>, InfrastructureError> {
        let file = sqlx::query_as::<_, File>("SELECT * FROM files WHERE s3_key = ?")
            .bind(s3_key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(file)
    }

    pub async fn create(&self, file: File) -> Result<(), InfrastructureError> {
        sqlx::query(
            "INSERT INTO files (id, name, parent_id, description, s3_key, file_size_bytes) VALUES (? ,? ,?, ?, ?, ?)",
        )
        .bind(&file.id)
        .bind(&file.name)
        .bind(&file.parent_id)
        .bind(&file.description)
        .bind(&file.s3_key)
        .bind(&file.file_size_bytes)
        .execute(&self.pool)
        .await
        .map_err(|e| InfrastructureError::from(e))?;

        Ok(())
    }

    pub async fn update_name(&self, id: i64, name: &str) -> Result<(), InfrastructureError> {
        let result = sqlx::query("UPDATE files SET name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        if result.rows_affected() == 0 {
            return Err(InfrastructureError::NotFound(format!("id {}", id)));
        }

        Ok(())
    }

    pub async fn update_description(
        &self,
        id: i64,
        description: &str,
    ) -> Result<(), InfrastructureError> {
        let result = sqlx::query("UPDATE files SET description = ? WHERE id = ?")
            .bind(description)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        if result.rows_affected() == 0 {
            return Err(InfrastructureError::NotFound(format!("id {}", id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), InfrastructureError> {
        let result = sqlx::query("DELETE FROM files WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        if result.rows_affected() == 0 {
            return Err(InfrastructureError::NotFound(format!("id {}", id)));
        }

        Ok(())
    }
}
