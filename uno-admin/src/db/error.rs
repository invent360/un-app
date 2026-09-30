//! Database error types for unity-dashboard.

use thiserror::Error;

/// Database-specific errors.
#[derive(Debug, Error)]
pub enum DbError {
    /// Connection error.
    #[error("Connection error: {0}")]
    Connection(String),

    /// Query execution error.
    #[error("Query error: {0}")]
    Query(String),

    /// Migration error.
    #[error("Migration error: {0}")]
    Migration(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Record not found.
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Duplicate record.
    #[error("Duplicate record: {0}")]
    Duplicate(String),

    /// Timeout error.
    #[error("Timeout: {0}")]
    Timeout(String),
}

/// Result type for database operations.
pub type DbResult<T> = Result<T, DbError>;

impl From<scylla::transport::errors::NewSessionError> for DbError {
    fn from(err: scylla::transport::errors::NewSessionError) -> Self {
        DbError::Connection(err.to_string())
    }
}

impl From<scylla::transport::errors::QueryError> for DbError {
    fn from(err: scylla::transport::errors::QueryError) -> Self {
        DbError::Query(err.to_string())
    }
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::Serialization(err.to_string())
    }
}

impl From<DbError> for String {
    fn from(err: DbError) -> Self {
        err.to_string()
    }
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        DbError::Query(err.to_string())
    }
}
