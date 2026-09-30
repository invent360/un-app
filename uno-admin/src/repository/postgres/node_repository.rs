//! PostgreSQL implementation of NodeRepositoryTrait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::models::entity::{NewNode, NodeEntity};
use crate::repository::traits::NodeRepositoryTrait;

/// Database row structure for nodes table
#[derive(sqlx::FromRow)]
struct NodeRow {
    id: Uuid,
    node_id: String,
    name: Option<String>,
    total_licenses: i32,
    online_licenses: i32,
    total_earnings_micros: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<NodeRow> for NodeEntity {
    fn from(row: NodeRow) -> Self {
        Self {
            id: row.id.to_string(),
            node_id: row.node_id,
            name: row.name,
            total_licenses: row.total_licenses,
            online_licenses: row.online_licenses,
            total_earnings_micros: row.total_earnings_micros,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
        }
    }
}

/// PostgreSQL implementation of the Node repository
pub struct PgNodeRepository {
    pool: Arc<PgPool>,
}

impl PgNodeRepository {
    /// Create a new PgNodeRepository with the given connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NodeRepositoryTrait for PgNodeRepository {
    async fn save_node(&self, new_node: NewNode) -> Result<NodeEntity, String> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let row = sqlx::query_as::<_, NodeRow>(
            r#"
            INSERT INTO nodes (
                id, node_id, name, total_licenses, online_licenses,
                total_earnings_micros, is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, node_id, name, total_licenses, online_licenses,
                      total_earnings_micros, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&new_node.node_id)
        .bind(&new_node.name)
        .bind(0i32)
        .bind(0i32)
        .bind(0i64)
        .bind(true)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to save node: {}", e);
            e.to_string()
        })?;

        debug!("Saved node: {}", row.id);

        Ok(row.into())
    }

    async fn get_node_by_id(&self, id: &str) -> Result<Option<NodeEntity>, String> {
        let uuid = Uuid::parse_str(id).map_err(|e| {
            error!("Invalid UUID: {}", e);
            e.to_string()
        })?;

        let row = sqlx::query_as::<_, NodeRow>(
            r#"
            SELECT id, node_id, name, total_licenses, online_licenses,
                   total_earnings_micros, created_at, updated_at
            FROM nodes
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(uuid)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get node by id: {}", e);
            e.to_string()
        })?;

        Ok(row.map(Into::into))
    }

    async fn get_node_by_node_id(&self, node_id: &str) -> Result<Option<NodeEntity>, String> {
        let row = sqlx::query_as::<_, NodeRow>(
            r#"
            SELECT id, node_id, name, total_licenses, online_licenses,
                   total_earnings_micros, created_at, updated_at
            FROM nodes
            WHERE node_id = $1 AND is_active = true
            "#,
        )
        .bind(node_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get node by node_id: {}", e);
            e.to_string()
        })?;

        Ok(row.map(Into::into))
    }

    async fn list_nodes(&self) -> Result<Vec<NodeEntity>, String> {
        let rows = sqlx::query_as::<_, NodeRow>(
            r#"
            SELECT id, node_id, name, total_licenses, online_licenses,
                   total_earnings_micros, created_at, updated_at
            FROM nodes
            WHERE is_active = true
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to list nodes: {}", e);
            e.to_string()
        })?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_node(&self, node: NodeEntity) -> Result<NodeEntity, String> {
        let uuid = Uuid::parse_str(&node.id).map_err(|e| {
            error!("Invalid UUID: {}", e);
            e.to_string()
        })?;

        let now = Utc::now();

        let row = sqlx::query_as::<_, NodeRow>(
            r#"
            UPDATE nodes
            SET node_id = $1, name = $2, total_licenses = $3,
                online_licenses = $4, total_earnings_micros = $5, updated_at = $6
            WHERE id = $7 AND is_active = true
            RETURNING id, node_id, name, total_licenses, online_licenses,
                      total_earnings_micros, created_at, updated_at
            "#,
        )
        .bind(&node.node_id)
        .bind(&node.name)
        .bind(node.total_licenses)
        .bind(node.online_licenses)
        .bind(node.total_earnings_micros)
        .bind(now)
        .bind(uuid)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to update node: {}", e);
            e.to_string()
        })?;

        debug!("Updated node: {}", row.id);

        Ok(row.into())
    }

    async fn delete_node(&self, id: &str) -> Result<bool, String> {
        let uuid = Uuid::parse_str(id).map_err(|e| {
            error!("Invalid UUID: {}", e);
            e.to_string()
        })?;

        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE nodes
            SET is_active = false, deleted_at = $1, updated_at = $1
            WHERE id = $2 AND is_active = true
            "#,
        )
        .bind(now)
        .bind(uuid)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to delete node: {}", e);
            e.to_string()
        })?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Soft-deleted node: {}", id);
        }

        Ok(deleted)
    }

    async fn count_nodes(&self) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM nodes
            WHERE is_active = true
            "#,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to count nodes: {}", e);
            e.to_string()
        })?;

        Ok(row.0)
    }

    async fn update_node_stats(
        &self,
        node_id: &str,
        total_licenses: i32,
        online_licenses: i32,
        total_earnings_micros: i64,
    ) -> Result<NodeEntity, String> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, NodeRow>(
            r#"
            UPDATE nodes
            SET total_licenses = $1, online_licenses = $2,
                total_earnings_micros = $3, updated_at = $4
            WHERE node_id = $5 AND is_active = true
            RETURNING id, node_id, name, total_licenses, online_licenses,
                      total_earnings_micros, created_at, updated_at
            "#,
        )
        .bind(total_licenses)
        .bind(online_licenses)
        .bind(total_earnings_micros)
        .bind(now)
        .bind(node_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to update node stats: {}", e);
            e.to_string()
        })?;

        match row {
            Some(r) => {
                debug!("Updated node stats for: {}", node_id);
                Ok(r.into())
            }
            None => Err(format!("Node not found: {}", node_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_row_conversion() {
        let now = Utc::now();
        let row = NodeRow {
            id: Uuid::new_v4(),
            node_id: "test-node-123".to_string(),
            name: Some("Test Node".to_string()),
            total_licenses: 10,
            online_licenses: 5,
            total_earnings_micros: 1_000_000,
            created_at: now,
            updated_at: now,
        };

        let entity: NodeEntity = row.into();

        assert_eq!(entity.node_id, "test-node-123");
        assert_eq!(entity.name, Some("Test Node".to_string()));
        assert_eq!(entity.total_licenses, 10);
        assert_eq!(entity.online_licenses, 5);
        assert_eq!(entity.total_earnings_micros, 1_000_000);
    }
}
