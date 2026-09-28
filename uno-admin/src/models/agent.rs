use serde::{Deserialize, Serialize};

/// Agent (recruiter who brings ULOs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub name: String,
    pub email: Option<String>,
    pub created_at: Option<String>,
}

impl Agent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: None,
            created_at: None,
        }
    }
}

/// Agent performance metrics (computed from rewards and licenses)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentPerformance {
    pub agent_name: String,
    pub ulo_count: usize,
    pub license_count: usize,
    pub total_earnings_micros: i64,   // Total rewards from licenses they recruited
    pub agent_share_micros: i64,      // Agent's commission
    pub allocations_count: usize,     // Number of reward allocations
    pub avg_uptime: f64,              // Average uptime of recruited ULOs' licenses
    pub online_licenses: usize,       // Count of online licenses
}

impl AgentPerformance {
    pub fn new(agent_name: impl Into<String>) -> Self {
        Self {
            agent_name: agent_name.into(),
            ..Default::default()
        }
    }

    /// Get total earnings in token units
    pub fn total_earnings(&self) -> f64 {
        self.total_earnings_micros as f64 / 1_000_000.0
    }

    /// Get agent's share in token units
    pub fn agent_share(&self) -> f64 {
        self.agent_share_micros as f64 / 1_000_000.0
    }

    /// Format total earnings
    pub fn total_earnings_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.total_earnings(), decimals)
    }

    /// Format agent share
    pub fn agent_share_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.agent_share(), decimals)
    }

    /// Get average uptime as percentage
    pub fn avg_uptime_percentage(&self) -> f64 {
        self.avg_uptime * 100.0
    }

    /// Format average uptime
    pub fn avg_uptime_formatted(&self) -> String {
        format!("{:.1}%", self.avg_uptime_percentage())
    }

    /// Get online percentage
    pub fn online_percentage(&self) -> f64 {
        if self.license_count == 0 {
            0.0
        } else {
            (self.online_licenses as f64 / self.license_count as f64) * 100.0
        }
    }
}

/// ULO (Unity License Operator) - person running the app
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ULO {
    pub name: String,           // "NGA Lagos1"
    pub agent_name: String,     // Agent who recruited them
    pub licenses: Vec<String>,  // List of license_ids they operate
}

impl ULO {
    pub fn new(name: impl Into<String>, agent_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agent_name: agent_name.into(),
            licenses: Vec::new(),
        }
    }

    /// Add a license to this ULO
    pub fn add_license(&mut self, license_id: String) {
        if !self.licenses.contains(&license_id) {
            self.licenses.push(license_id);
        }
    }

    /// Get license count
    pub fn license_count(&self) -> usize {
        self.licenses.len()
    }
}

/// Agent summary for dashboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentSummary {
    pub total_agents: usize,
    pub total_ulos: usize,
    pub total_agent_share_micros: i64,
    pub top_agent: Option<String>,
    pub top_agent_earnings_micros: i64,
}

impl AgentSummary {
    pub fn total_agent_share(&self) -> f64 {
        self.total_agent_share_micros as f64 / 1_000_000.0
    }

    pub fn top_agent_earnings(&self) -> f64 {
        self.top_agent_earnings_micros as f64 / 1_000_000.0
    }
}
