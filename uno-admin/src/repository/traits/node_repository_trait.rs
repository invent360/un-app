//! Node repository trait definition

use async_trait::async_trait;
use crate::models::entity::{NodeEntity, NewNode};

/// Repository trait for Node operations
#[async_trait]
pub trait NodeRepositoryTrait: Send + Sync {
    /// Save a new node to the database
    async fn save_node(&self, new_node: NewNode) -> Result<NodeEntity, String>;

    /// Get a node by its primary key (UUID)
    async fn get_node_by_id(&self, id: &str) -> Result<Option<NodeEntity>, String>;

    /// Get a node by its node_id (blockchain ID)
    async fn get_node_by_node_id(&self, node_id: &str) -> Result<Option<NodeEntity>, String>;

    /// List all nodes
    async fn list_nodes(&self) -> Result<Vec<NodeEntity>, String>;

    /// Update an existing node
    async fn update_node(&self, node: NodeEntity) -> Result<NodeEntity, String>;

    /// Delete a node by ID
    async fn delete_node(&self, id: &str) -> Result<bool, String>;

    /// Count all nodes
    async fn count_nodes(&self) -> Result<i64, String>;

    /// Update node statistics (license counts, earnings)
    async fn update_node_stats(
        &self,
        node_id: &str,
        total_licenses: i32,
        online_licenses: i32,
        total_earnings_micros: i64,
    ) -> Result<NodeEntity, String>;
}
