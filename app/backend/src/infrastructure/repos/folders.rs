use sqlx::SqlitePool;

use crate::{
    infrastructure::{dto::CreateFolderDTO, errors::InfrastructureError},
    models::folder::Folder,
};

#[derive(Clone)]
pub struct SQLiteFolderRepository {
    pool: SqlitePool,
}

impl SQLiteFolderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        return Self { pool };
    }

    pub async fn get_all(&self, name: &str) -> Result<Vec<Folder>, InfrastructureError> {
        let folders = sqlx::query_as::<_, Folder>("SELECT * FROM folders")
            .bind(name)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(folders)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<Folder>, InfrastructureError> {
        let folder = sqlx::query_as::<_, Folder>("SELECT * FROM folders WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(folder)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Folder>, InfrastructureError> {
        let folder = sqlx::query_as::<_, Folder>("SELECT * FROM folders WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(folder)
    }

    pub async fn get_by_s3_key(&self, s3_key: &str) -> Result<Option<Folder>, InfrastructureError> {
        let folder = sqlx::query_as::<_, Folder>("SELECT * FROM folders WHERE s3_key = ?")
            .bind(s3_key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(folder)
    }

    pub async fn create(&self, payload: CreateFolderDTO) -> Result<(), InfrastructureError> {
        sqlx::query("INSERT INTO folders (name, parent_id) VALUES (?,?)")
            .bind(&payload.name)
            .bind(&payload.parent_id)
            .execute(&self.pool)
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        Ok(())
    }

    pub async fn update_name(&self, id: i64, name: &str) -> Result<(), InfrastructureError> {
        let result = sqlx::query("UPDATE folders SET name = ? WHERE id = ?")
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

    pub async fn update_position(
        &self,
        id: i64,
        parent_id: i64,
    ) -> Result<(), InfrastructureError> {
        let result = sqlx::query("UPDATE folders SET parent_id = ? WHERE id = ?")
            .bind(parent_id)
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
        let result = sqlx::query("DELETE FROM folders WHERE id = ?")
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
