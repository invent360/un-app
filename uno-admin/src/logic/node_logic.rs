//! Business logic for node operations
//!
//! Supports both ScyllaDB (legacy) and PostgreSQL backends via feature flags.

use crate::db::DbPool;
use crate::models::entity::{LicenseEntity, NewNode, NodeEntity};
use crate::repository::traits::{LicenseRepositoryTrait, NodeRepositoryTrait};

// Import the appropriate repositories based on feature flag
#[cfg(feature = "postgres-db")]
use crate::repository::postgres::{PgLicenseRepository as LicenseRepository, PgNodeRepository as NodeRepository};
#[cfg(not(feature = "postgres-db"))]
use crate::repository::scylla::{LicenseRepository, NodeRepository};

/// Node service for business operations
pub struct NodeService {
    node_repo: NodeRepository,
    license_repo: LicenseRepository,
}

impl NodeService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            node_repo: NodeRepository::new(pool.clone()),
            license_repo: LicenseRepository::new(pool),
        }
    }

    /// Create a new node
    pub async fn create_node(&self, new_node: NewNode) -> Result<NodeEntity, String> {
        // Check if node_id already exists
        if let Some(_) = self
            .node_repo
            .get_node_by_node_id(&new_node.node_id)
            .await
            .map_err(|e| e.to_string())?
        {
            return Err(format!("Node with ID '{}' already exists", new_node.node_id));
        }

        self.node_repo
            .save_node(new_node)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get or create a node by node_id
    pub async fn get_or_create_node(&self, node_id: &str) -> Result<NodeEntity, String> {
        if let Some(node) = self
            .node_repo
            .get_node_by_node_id(node_id)
            .await
            .map_err(|e| e.to_string())?
        {
            return Ok(node);
        }

        let new_node = NewNode::new(node_id.to_string());
        self.node_repo
            .save_node(new_node)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get all nodes
    pub async fn list_nodes(&self) -> Result<Vec<NodeEntity>, String> {
        self.node_repo
            .list_nodes()
            .await
            .map_err(|e| e.to_string())
    }

    /// Get a node by ID
    pub async fn get_node(&self, id: &str) -> Result<Option<NodeEntity>, String> {
        self.node_repo
            .get_node_by_id(id)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get a node by node_id
    pub async fn get_node_by_node_id(&self, node_id: &str) -> Result<Option<NodeEntity>, String> {
        self.node_repo
            .get_node_by_node_id(node_id)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get a node with its licenses
    pub async fn get_node_with_licenses(
        &self,
        node_id: &str,
    ) -> Result<Option<(NodeEntity, Vec<LicenseEntity>)>, String> {
        let node = self
            .node_repo
            .get_node_by_node_id(node_id)
            .await
            .map_err(|e| e.to_string())?;

        match node {
            Some(node) => {
                let licenses = self
                    .license_repo
                    .get_licenses_by_node_id(&node.node_id)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Some((node, licenses)))
            }
            None => Ok(None),
        }
    }

    /// Update a node
    pub async fn update_node(&self, node: NodeEntity) -> Result<NodeEntity, String> {
        self.node_repo
            .update_node(node)
            .await
            .map_err(|e| e.to_string())
    }

    /// Delete a node
    pub async fn delete_node(&self, id: &str) -> Result<bool, String> {
        self.node_repo
            .delete_node(id)
            .await
            .map_err(|e| e.to_string())
    }

    /// Count nodes
    pub async fn count_nodes(&self) -> Result<i64, String> {
        self.node_repo
            .count_nodes()
            .await
            .map_err(|e| e.to_string())
    }

    /// Update node statistics based on its licenses
    pub async fn refresh_node_stats(&self, node_id: &str) -> Result<NodeEntity, String> {
        let licenses = self
            .license_repo
            .get_licenses_by_node_id(node_id)
            .await
            .map_err(|e| e.to_string())?;

        let total_licenses = licenses.len() as i32;
        let online_licenses = licenses.iter().filter(|l| l.is_online).count() as i32;

        // Note: total_earnings_micros would need to be computed from rewards data
        // For now, we just update license counts
        self.node_repo
            .update_node_stats(node_id, total_licenses, online_licenses, 0)
            .await
            .map_err(|e| e.to_string())
    }

    /// Sync nodes from license data - ensures nodes exist for all licenses
    pub async fn sync_nodes_from_licenses(&self) -> Result<usize, String> {
        let licenses = self
            .license_repo
            .list_licenses()
            .await
            .map_err(|e| e.to_string())?;

        // Get unique node_ids
        let mut node_ids: Vec<String> = licenses.iter().map(|l| l.node_id.clone()).collect();
        node_ids.sort();
        node_ids.dedup();

        let mut created_count = 0;

        for node_id in &node_ids {
            if self.get_node_by_node_id(node_id).await?.is_none() {
                self.create_node(NewNode::new(node_id.clone())).await?;
                created_count += 1;
            }
        }

        // Refresh stats for all nodes
        for node_id in &node_ids {
            self.refresh_node_stats(node_id).await?;
        }

        Ok(created_count)
    }

    /// Get node statistics summary
    pub async fn get_nodes_summary(&self) -> Result<NodesSummary, String> {
        let nodes = self.list_nodes().await?;

        let total_nodes = nodes.len();
        let total_licenses: i32 = nodes.iter().map(|n| n.total_licenses).sum();
        let total_online: i32 = nodes.iter().map(|n| n.online_licenses).sum();
        let total_earnings: i64 = nodes.iter().map(|n| n.total_earnings_micros).sum();

        Ok(NodesSummary {
            total_nodes,
            total_licenses: total_licenses as usize,
            total_online: total_online as usize,
            total_earnings_micros: total_earnings,
        })
    }
}

/// Nodes summary statistics
#[derive(Debug, Clone, Default)]
pub struct NodesSummary {
    pub total_nodes: usize,
    pub total_licenses: usize,
    pub total_online: usize,
    pub total_earnings_micros: i64,
}

impl NodesSummary {
    pub fn total_earnings(&self) -> f64 {
        self.total_earnings_micros as f64 / 1_000_000.0
    }

    pub fn online_percentage(&self) -> f64 {
        if self.total_licenses == 0 {
            0.0
        } else {
            (self.total_online as f64 / self.total_licenses as f64) * 100.0
        }
    }
}
