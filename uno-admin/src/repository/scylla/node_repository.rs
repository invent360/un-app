//! ScyllaDB implementation of NodeRepositoryTrait

use async_trait::async_trait;
use chrono::Utc;
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::entity::{NodeEntity, NewNode};
use crate::repository::traits::NodeRepositoryTrait;

type NodeRow = (String, String, Option<String>, i32, i32, i64, CqlTimestamp, CqlTimestamp);

/// ScyllaDB implementation of the Node repository
pub struct NodeRepository {
    session: Arc<Session>,
}

impl NodeRepository {
    /// Create a new NodeRepository with the given session
    pub fn new(session: DbPool) -> Self {
        Self { session }
    }

    fn timestamp_to_datetime(ts: CqlTimestamp) -> String {
        chrono::DateTime::from_timestamp_millis(ts.0)
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }

    fn parse_single_node(result: scylla::QueryResult) -> Result<Option<NodeEntity>, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<NodeRow>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        for row_result in rows {
            if let Ok((id, node_id, name, total_licenses, online_licenses, total_earnings_micros, created_at, updated_at)) = row_result {
                return Ok(Some(NodeEntity {
                    id,
                    node_id,
                    name,
                    total_licenses,
                    online_licenses,
                    total_earnings_micros,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                }));
            }
        }

        Ok(None)
    }

    fn parse_nodes(result: scylla::QueryResult) -> Result<Vec<NodeEntity>, String> {
        let mut nodes = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(nodes),
        };

        let rows = match rows_result.rows::<NodeRow>() {
            Ok(r) => r,
            Err(_) => return Ok(nodes),
        };

        for row_result in rows {
            if let Ok((id, node_id, name, total_licenses, online_licenses, total_earnings_micros, created_at, updated_at)) = row_result {
                nodes.push(NodeEntity {
                    id,
                    node_id,
                    name,
                    total_licenses,
                    online_licenses,
                    total_earnings_micros,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                });
            }
        }

        Ok(nodes)
    }
}

#[async_trait]
impl NodeRepositoryTrait for NodeRepository {
    async fn save_node(&self, new_node: NewNode) -> Result<NodeEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());
        let id = Uuid::new_v4().to_string();

        let query = "INSERT INTO nodes (id, node_id, name, total_licenses, online_licenses, total_earnings_micros, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.session
            .query_unpaged(
                query,
                (
                    &id,
                    &new_node.node_id,
                    &new_node.name,
                    0i32,
                    0i32,
                    0i64,
                    true,
                    now_ts,
                    now_ts,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to save node: {}", e);
                e.to_string()
            })?;

        debug!("Saved node: {}", id);

        Ok(NodeEntity {
            id,
            node_id: new_node.node_id,
            name: new_node.name,
            total_licenses: 0,
            online_licenses: 0,
            total_earnings_micros: 0,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
        })
    }

    async fn get_node_by_id(&self, id: &str) -> Result<Option<NodeEntity>, String> {
        let query = "SELECT id, node_id, name, total_licenses, online_licenses, total_earnings_micros, created_at, updated_at FROM nodes WHERE id = ?";

        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_single_node(result)
    }

    async fn get_node_by_node_id(&self, node_id: &str) -> Result<Option<NodeEntity>, String> {
        let query = "SELECT id, node_id, name, total_licenses, online_licenses, total_earnings_micros, created_at, updated_at FROM nodes WHERE node_id = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (node_id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_single_node(result)
    }

    async fn list_nodes(&self) -> Result<Vec<NodeEntity>, String> {
        let query = "SELECT id, node_id, name, total_licenses, online_licenses, total_earnings_micros, created_at, updated_at FROM nodes WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_nodes(result)
    }

    async fn update_node(&self, node: NodeEntity) -> Result<NodeEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());

        let query = "UPDATE nodes SET node_id = ?, name = ?, total_licenses = ?, online_licenses = ?, total_earnings_micros = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    &node.node_id,
                    &node.name,
                    node.total_licenses,
                    node.online_licenses,
                    node.total_earnings_micros,
                    now_ts,
                    &node.id,
                ),
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(NodeEntity {
            updated_at: now.to_rfc3339(),
            ..node
        })
    }

    async fn delete_node(&self, id: &str) -> Result<bool, String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE nodes SET is_active = false, deleted_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, (now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }

    async fn count_nodes(&self) -> Result<i64, String> {
        let query = "SELECT COUNT(*) FROM nodes WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let rows = match rows_result.rows::<(i64,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        for row_result in rows {
            if let Ok((count,)) = row_result {
                return Ok(count);
            }
        }

        Ok(0)
    }

    async fn update_node_stats(
        &self,
        node_id: &str,
        total_licenses: i32,
        online_licenses: i32,
        total_earnings_micros: i64,
    ) -> Result<NodeEntity, String> {
        // First get the node
        let node = self
            .get_node_by_node_id(node_id)
            .await?
            .ok_or_else(|| format!("Node not found: {}", node_id))?;

        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());

        let query = "UPDATE nodes SET total_licenses = ?, online_licenses = ?, total_earnings_micros = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    total_licenses,
                    online_licenses,
                    total_earnings_micros,
                    now_ts,
                    &node.id,
                ),
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(NodeEntity {
            total_licenses,
            online_licenses,
            total_earnings_micros,
            updated_at: now.to_rfc3339(),
            ..node
        })
    }
}
