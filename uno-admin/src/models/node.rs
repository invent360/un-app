use serde::{Deserialize, Serialize};

/// Node containing multiple licenses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub node_id: String,
    pub licenses: Vec<String>,  // license_ids
    pub total_earnings_micros: i64,
    pub online_count: usize,
    pub total_count: usize,
}

impl Node {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            licenses: Vec::new(),
            total_earnings_micros: 0,
            online_count: 0,
            total_count: 0,
        }
    }

    /// Get total earnings in token units
    pub fn total_earnings(&self) -> f64 {
        self.total_earnings_micros as f64 / 1_000_000.0
    }

    /// Format earnings with specified decimal places
    pub fn earnings_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.total_earnings(), decimals)
    }

    /// Get shortened node ID (first 8 + last 6 chars)
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!("{}...{}", &self.node_id[..10], &self.node_id[self.node_id.len()-6..])
        } else {
            self.node_id.clone()
        }
    }

    /// Get online percentage
    pub fn online_percentage(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.online_count as f64 / self.total_count as f64) * 100.0
        }
    }
}

/// Node summary for dashboard display
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeSummary {
    pub total_nodes: usize,
    pub total_licenses: usize,
    pub total_online: usize,
    pub total_earnings_micros: i64,
}

impl NodeSummary {
    pub fn total_earnings(&self) -> f64 {
        self.total_earnings_micros as f64 / 1_000_000.0
    }
}
