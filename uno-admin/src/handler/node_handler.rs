//! Handlers for node operations

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::entity::{NodeEntity, LicenseEntity};

/// Server function to list all nodes
#[server(ListNodes, "/api")]
pub async fn list_nodes() -> Result<Vec<NodeEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::NodeService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = NodeService::new(pool);
    service
        .list_nodes()
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to get a single node by node_id
#[server(GetNode, "/api")]
pub async fn get_node(node_id: String) -> Result<Option<NodeEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::NodeService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = NodeService::new(pool);
    service
        .get_node_by_node_id(&node_id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to get node with its licenses
#[server(GetNodeWithLicenses, "/api")]
pub async fn get_node_with_licenses(
    node_id: String,
) -> Result<Option<(NodeEntity, Vec<LicenseEntity>)>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::NodeService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = NodeService::new(pool);
    service
        .get_node_with_licenses(&node_id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Nodes summary statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodesSummaryDto {
    pub total_nodes: usize,
    pub total_licenses: usize,
    pub total_online: usize,
    pub total_earnings_micros: i64,
}

/// Server function to get nodes summary
#[server(GetNodesSummary, "/api")]
pub async fn get_nodes_summary() -> Result<NodesSummaryDto, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::NodeService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = NodeService::new(pool);
    let summary = service
        .get_nodes_summary()
        .await
        .map_err(|e| ServerFnError::new(e))?;

    Ok(NodesSummaryDto {
        total_nodes: summary.total_nodes,
        total_licenses: summary.total_licenses,
        total_online: summary.total_online,
        total_earnings_micros: summary.total_earnings_micros,
    })
}

/// Server function to sync nodes from license data
#[server(SyncNodes, "/api")]
pub async fn sync_nodes() -> Result<usize, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::NodeService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = NodeService::new(pool);
    service
        .sync_nodes_from_licenses()
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// State for the nodes page
#[derive(Debug, Clone, Default)]
pub struct NodesPageState {
    pub nodes: Vec<NodeEntity>,
    pub summary: NodesSummaryDto,
    pub loading: bool,
    pub error: Option<String>,
}

/// Context for managing nodes state
#[derive(Clone, Copy)]
pub struct NodesContext {
    pub state: RwSignal<NodesPageState>,
}

impl NodesContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(NodesPageState::default()),
        }
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.loading = loading);
    }

    pub fn set_error(&self, error: Option<String>) {
        self.state.update(|s| s.error = error);
    }

    pub fn set_nodes(&self, nodes: Vec<NodeEntity>) {
        self.state.update(|s| {
            s.nodes = nodes;
            s.loading = false;
            s.error = None;
        });
    }

    pub fn set_summary(&self, summary: NodesSummaryDto) {
        self.state.update(|s| s.summary = summary);
    }

    pub fn get_node(&self, node_id: &str) -> Option<NodeEntity> {
        self.state.get().nodes.iter()
            .find(|n| n.node_id == node_id)
            .cloned()
    }
}

impl Default for NodesContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook to access nodes context
pub fn use_nodes() -> NodesContext {
    expect_context::<NodesContext>()
}

/// Provider component for NodesContext
#[component]
pub fn NodesContextProvider(children: Children) -> impl IntoView {
    let context = NodesContext::new();
    provide_context(context);
    children()
}
