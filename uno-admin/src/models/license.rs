use serde::{Deserialize, Serialize};

/// Split percentages for a license
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaseSplit {
    pub uno_share: f64,    // e.g., 47.0 (percent)
    pub agent_share: f64,  // e.g., 3.0 (percent)
    pub ulo_share: f64,    // e.g., 50.0 (percent)
}

impl LeaseSplit {
    pub fn new(uno_share: f64, agent_share: f64, ulo_share: f64) -> Self {
        Self { uno_share, agent_share, ulo_share }
    }

    /// Validate that splits sum to 100%
    pub fn validate(&self) -> bool {
        (self.uno_share + self.agent_share + self.ulo_share - 100.0).abs() < 0.01
    }

    /// Get agent's portion of UNO's share
    /// If splits are uno:47%, agent:3% → agent gets 3/(47+3) = 6% of UNO's portion
    pub fn agent_portion_of_uno(&self) -> f64 {
        let uno_total = self.uno_share + self.agent_share;
        if uno_total == 0.0 {
            0.0
        } else {
            self.agent_share / uno_total
        }
    }
}

/// License configuration from CSV merged with API data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicenseConfig {
    pub license_id: String,
    pub node_id: String,           // From API
    pub lease_code: String,        // From CSV or API
    pub alias: String,             // From API
    pub splits: LeaseSplit,
    pub ulo_name: String,          // From CSV (operator name)
    pub agent_name: String,        // From CSV (recruiter name)
    pub uptime: f64,               // From API (0.0 to 1.0)
    pub is_online: bool,           // From API
    pub device_name: Option<String>,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
}

impl Default for LicenseConfig {
    fn default() -> Self {
        Self {
            license_id: String::new(),
            node_id: String::new(),
            lease_code: String::new(),
            alias: String::new(),
            splits: LeaseSplit::default(),
            ulo_name: String::new(),
            agent_name: String::new(),
            uptime: 0.0,
            is_online: false,
            device_name: None,
            lease_from: None,
            lease_to: None,
        }
    }
}

impl LicenseConfig {
    /// Get shortened license ID
    pub fn license_id_short(&self) -> String {
        if self.license_id.len() > 16 {
            format!("{}...{}", &self.license_id[..10], &self.license_id[self.license_id.len()-6..])
        } else {
            self.license_id.clone()
        }
    }

    /// Get shortened node ID
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!("{}...{}", &self.node_id[..10], &self.node_id[self.node_id.len()-6..])
        } else {
            self.node_id.clone()
        }
    }

    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }

    /// Format uptime as percentage string
    pub fn uptime_formatted(&self) -> String {
        format!("{:.1}%", self.uptime_percentage())
    }

    /// Check if license has an agent assigned
    pub fn has_agent(&self) -> bool {
        !self.agent_name.is_empty()
    }
}

/// CSV row for license configuration import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseConfigCsv {
    pub license_id: String,
    pub lease_code: String,
    pub uno_share: String,    // "47%" format
    pub agent_share: String,  // "3%" format
    pub ulo_share: String,    // "50%" format
    pub ulo: String,          // ULO name
    pub agent: String,        // Agent name
}

impl LicenseConfigCsv {
    /// Parse percentage string like "47%" to f64
    pub fn parse_percentage(s: &str) -> Result<f64, String> {
        let cleaned = s.trim().replace('%', "");
        cleaned.parse::<f64>().map_err(|_| format!("Invalid percentage: {}", s))
    }

    /// Convert CSV row to LicenseConfig (partial, needs API data merge)
    pub fn to_config(&self) -> Result<LicenseConfig, String> {
        let uno_share = Self::parse_percentage(&self.uno_share)?;
        let agent_share = Self::parse_percentage(&self.agent_share)?;
        let ulo_share = Self::parse_percentage(&self.ulo_share)?;

        Ok(LicenseConfig {
            license_id: self.license_id.trim().to_string(),
            node_id: String::new(),  // Will be filled from API
            lease_code: self.lease_code.trim().to_string(),
            alias: String::new(),     // Will be filled from API
            splits: LeaseSplit::new(uno_share, agent_share, ulo_share),
            ulo_name: self.ulo.trim().to_string(),
            agent_name: self.agent.trim().to_string(),
            uptime: 0.0,             // Will be filled from API
            is_online: false,        // Will be filled from API
            device_name: None,
            lease_from: None,
            lease_to: None,
        })
    }
}

/// License summary statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LicenseSummary {
    pub total_licenses: usize,
    pub online_licenses: usize,
    pub total_earnings_micros: i64,
    pub total_uno_share_micros: i64,
    pub total_agent_share_micros: i64,
    pub avg_uptime: f64,
}

impl LicenseSummary {
    pub fn total_earnings(&self) -> f64 {
        self.total_earnings_micros as f64 / 1_000_000.0
    }

    pub fn uno_earnings(&self) -> f64 {
        self.total_uno_share_micros as f64 / 1_000_000.0
    }

    pub fn agent_earnings(&self) -> f64 {
        self.total_agent_share_micros as f64 / 1_000_000.0
    }

    pub fn online_percentage(&self) -> f64 {
        if self.total_licenses == 0 {
            0.0
        } else {
            (self.online_licenses as f64 / self.total_licenses as f64) * 100.0
        }
    }
}
