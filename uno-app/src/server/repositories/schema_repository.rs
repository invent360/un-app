//! Schema repository for database operations on content schemas

use async_trait::async_trait;
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{AppError, ContentSchema};

/// Dynamic type alias for SchemaRepository trait object
pub type DynSchemaRepository = Arc<dyn SchemaRepository + Send + Sync>;

/// Schema repository trait defining database operations
#[async_trait]
pub trait SchemaRepository {
    /// Get all schemas
    async fn get_all(&self) -> Result<Vec<ContentSchema>, AppError>;

    /// Get schema by ID
    async fn get_by_id(&self, id: &str) -> Result<Option<ContentSchema>, AppError>;

    /// Create a new schema
    async fn create(&self, schema: &ContentSchema) -> Result<ContentSchema, AppError>;

    /// Update an existing schema
    async fn update(&self, id: &str, schema: &ContentSchema) -> Result<ContentSchema, AppError>;

    /// Delete a schema (only if no content items reference it)
    async fn delete(&self, id: &str) -> Result<(), AppError>;

    /// Check if schema has any content items
    async fn has_content(&self, id: &str) -> Result<bool, AppError>;

    /// Get system schemas (non-deletable)
    async fn get_system_schemas(&self) -> Result<Vec<ContentSchema>, AppError>;
}

/// Concrete implementation of SchemaRepository
pub struct SchemaRepositoryImpl {
    db_pool: ConnectionPool,
}

impl SchemaRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl SchemaRepository for SchemaRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<ContentSchema>, AppError> {
        let schemas = sqlx::query_as::<_, ContentSchema>(
            r#"
            SELECT id, name, name_plural, description, icon,
                   fields, settings, version, is_system,
                   created_at, updated_at
            FROM content_schemas
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(schemas)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<ContentSchema>, AppError> {
        let schema = sqlx::query_as::<_, ContentSchema>(
            r#"
            SELECT id, name, name_plural, description, icon,
                   fields, settings, version, is_system,
                   created_at, updated_at
            FROM content_schemas
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(schema)
    }

    async fn create(&self, schema: &ContentSchema) -> Result<ContentSchema, AppError> {
        let created = sqlx::query_as::<_, ContentSchema>(
            r#"
            INSERT INTO content_schemas (
                id, name, name_plural, description, icon,
                fields, settings, is_system
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, name_plural, description, icon,
                      fields, settings, version, is_system,
                      created_at, updated_at
            "#
        )
        .bind(&schema.id)
        .bind(&schema.name)
        .bind(&schema.name_plural)
        .bind(&schema.description)
        .bind(&schema.icon)
        .bind(serde_json::to_value(&schema.fields).unwrap_or_default())
        .bind(serde_json::to_value(&schema.settings).unwrap_or_default())
        .bind(schema.is_system)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(created)
    }

    async fn update(&self, id: &str, schema: &ContentSchema) -> Result<ContentSchema, AppError> {
        // Check if schema exists
        let existing = self.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", id)))?;

        let new_version = existing.version + 1;

        let updated = sqlx::query_as::<_, ContentSchema>(
            r#"
            UPDATE content_schemas
            SET name = $2, name_plural = $3, description = $4, icon = $5,
                fields = $6, settings = $7, version = $8, updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, name_plural, description, icon,
                      fields, settings, version, is_system,
                      created_at, updated_at
            "#
        )
        .bind(id)
        .bind(&schema.name)
        .bind(&schema.name_plural)
        .bind(&schema.description)
        .bind(&schema.icon)
        .bind(serde_json::to_value(&schema.fields).unwrap_or_default())
        .bind(serde_json::to_value(&schema.settings).unwrap_or_default())
        .bind(new_version)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<(), AppError> {
        // Check if schema exists and is not a system schema
        let schema = self.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", id)))?;

        if schema.is_system {
            return Err(AppError::BadRequest("Cannot delete system schema".into()));
        }

        // Check if schema has content
        if self.has_content(id).await? {
            return Err(AppError::BadRequest(
                "Cannot delete schema with existing content items. Delete or migrate content first.".into()
            ));
        }

        sqlx::query("DELETE FROM content_schemas WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    async fn has_content(&self, id: &str) -> Result<bool, AppError> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM content_items WHERE schema_id = $1"
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(count.0 > 0)
    }

    async fn get_system_schemas(&self) -> Result<Vec<ContentSchema>, AppError> {
        let schemas = sqlx::query_as::<_, ContentSchema>(
            r#"
            SELECT id, name, name_plural, description, icon,
                   fields, settings, version, is_system,
                   created_at, updated_at
            FROM content_schemas
            WHERE is_system = true
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(schemas)
    }
}
