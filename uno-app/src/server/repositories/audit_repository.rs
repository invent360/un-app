//! Audit log repository for database operations

use async_trait::async_trait;
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{AppError, AuditLogEntry, CreateAuditLog, AuditLogFilter, AuditLogResponse};

/// Dynamic type alias for AuditRepository trait object
pub type DynAuditRepository = Arc<dyn AuditRepository + Send + Sync>;

/// Audit repository trait defining database operations
#[async_trait]
pub trait AuditRepository: Send + Sync {
    /// Create a new audit log entry
    async fn create(&self, entry: CreateAuditLog) -> Result<AuditLogEntry, AppError>;

    /// List audit logs with filtering and pagination
    async fn list(&self, filter: AuditLogFilter) -> Result<AuditLogResponse, AppError>;

    /// Get audit logs for a specific entity
    async fn get_by_entity(&self, entity_type: &str, entity_id: i32) -> Result<Vec<AuditLogEntry>, AppError>;
}

/// Concrete implementation of AuditRepository
pub struct AuditRepositoryImpl {
    db_pool: ConnectionPool,
}

impl AuditRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AuditRepository for AuditRepositoryImpl {
    async fn create(&self, entry: CreateAuditLog) -> Result<AuditLogEntry, AppError> {
        let item = sqlx::query_as::<_, AuditLogEntry>(
            r#"
            INSERT INTO audit_logs
                (entity_type, entity_id, action, actor_id, actor_name, old_values, new_values, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, entity_type, entity_id, action, actor_id, actor_name,
                      old_values, new_values, metadata, created_at
            "#
        )
        .bind(&entry.entity_type)
        .bind(entry.entity_id)
        .bind(&entry.action)
        .bind(&entry.actor_id)
        .bind(&entry.actor_name)
        .bind(&entry.old_values)
        .bind(&entry.new_values)
        .bind(&entry.metadata)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn list(&self, filter: AuditLogFilter) -> Result<AuditLogResponse, AppError> {
        let offset = ((filter.page - 1) * filter.per_page) as i64;
        let limit = filter.per_page as i64;

        // Count query with filters
        let total: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM audit_logs
            WHERE ($1::text IS NULL OR entity_type = $1)
              AND ($2::int IS NULL OR entity_id = $2)
              AND ($3::text IS NULL OR action = $3)
              AND ($4::text IS NULL OR actor_id = $4)
              AND ($5::timestamptz IS NULL OR created_at >= $5)
              AND ($6::timestamptz IS NULL OR created_at <= $6)
            "#
        )
        .bind(&filter.entity_type)
        .bind(filter.entity_id)
        .bind(&filter.action)
        .bind(&filter.actor_id)
        .bind(filter.from_date)
        .bind(filter.to_date)
        .fetch_one(&self.db_pool)
        .await?;

        // Data query with filters
        let entries = sqlx::query_as::<_, AuditLogEntry>(
            r#"
            SELECT id, entity_type, entity_id, action, actor_id, actor_name,
                   old_values, new_values, metadata, created_at
            FROM audit_logs
            WHERE ($1::text IS NULL OR entity_type = $1)
              AND ($2::int IS NULL OR entity_id = $2)
              AND ($3::text IS NULL OR action = $3)
              AND ($4::text IS NULL OR actor_id = $4)
              AND ($5::timestamptz IS NULL OR created_at >= $5)
              AND ($6::timestamptz IS NULL OR created_at <= $6)
            ORDER BY created_at DESC
            LIMIT $7 OFFSET $8
            "#
        )
        .bind(&filter.entity_type)
        .bind(filter.entity_id)
        .bind(&filter.action)
        .bind(&filter.actor_id)
        .bind(filter.from_date)
        .bind(filter.to_date)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(AuditLogResponse {
            entries,
            total: total.0,
            page: filter.page,
            per_page: filter.per_page,
        })
    }

    async fn get_by_entity(&self, entity_type: &str, entity_id: i32) -> Result<Vec<AuditLogEntry>, AppError> {
        let entries = sqlx::query_as::<_, AuditLogEntry>(
            r#"
            SELECT id, entity_type, entity_id, action, actor_id, actor_name,
                   old_values, new_values, metadata, created_at
            FROM audit_logs
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY created_at DESC
            LIMIT 100
            "#
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(entries)
    }
}
