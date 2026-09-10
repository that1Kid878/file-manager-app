use sqlx::error::ErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Pool connection closed: {0}")]
    PoolClosed(String),

    #[error("No connection from pool:{0}")]
    PoolTimeOut(String),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Unique constraint violated: {0}")]
    UniqueViolation(String),

    #[error("Not null constraint violated: {0}")]
    NotNullViolation(String),

    #[error("Foreign key constraint violated: {0}")]
    ForeignKeyViolation(String),

    #[error("IO operation failed: {0}")]
    IoError(String),

    #[error("TLS failed: {0}")]
    TlsError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolClosed => InfrastructureError::PoolClosed(err.to_string()),
            sqlx::Error::PoolTimedOut => InfrastructureError::PoolTimeOut(err.to_string()),
            sqlx::Error::RowNotFound => InfrastructureError::NotFound(err.to_string()),
            sqlx::Error::Database(db_err) => match db_err.kind() {
                ErrorKind::NotNullViolation => {
                    InfrastructureError::NotNullViolation(db_err.message().to_string())
                }
                ErrorKind::UniqueViolation => {
                    InfrastructureError::UniqueViolation(db_err.message().to_string())
                }
                ErrorKind::ForeignKeyViolation => {
                    InfrastructureError::ForeignKeyViolation(db_err.message().to_string())
                }
                _ => InfrastructureError::DatabaseError(db_err.message().to_string()),
            },
            sqlx::Error::Io(io_err) => InfrastructureError::IoError(io_err.to_string()),
            sqlx::Error::Tls(tls_err) => InfrastructureError::TlsError(tls_err.to_string()),
            _ => InfrastructureError::DatabaseError(err.to_string()),
        }
    }
}
