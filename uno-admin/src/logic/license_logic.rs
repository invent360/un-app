//! Business logic for license operations

use crate::api::types::{LicenseApi, UptimeAnalyticsPoint};
use crate::models::{LicenseConfig, LeaseSplit, Node};
use std::collections::HashMap;

/// Parse license configuration from CSV text
/// Expected format: license_id,lease_code,uno_share,agent_share,ulo_share,ulo,agent
pub fn parse_license_csv(csv_text: &str) -> Result<Vec<LicenseConfig>, String> {
    let mut configs = Vec::new();
    let lines: Vec<&str> = csv_text.lines().collect();

    if lines.is_empty() {
        return Err("Empty CSV file".to_string());
    }

    // Skip header line
    for (line_num, line) in lines.iter().enumerate().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 7 {
            return Err(format!("Line {}: Expected 7 fields, got {}", line_num + 1, fields.len()));
        }

        let license_id = fields[0].trim().to_string();
        let lease_code = fields[1].trim().to_string();

        // Parse percentages (remove % sign if present)
        let uno_share = parse_percentage(fields[2].trim())
            .map_err(|_| format!("Line {}: Invalid uno_share '{}'", line_num + 1, fields[2]))?;
        let agent_share = parse_percentage(fields[3].trim())
            .map_err(|_| format!("Line {}: Invalid agent_share '{}'", line_num + 1, fields[3]))?;
        let ulo_share = parse_percentage(fields[4].trim())
            .map_err(|_| format!("Line {}: Invalid ulo_share '{}'", line_num + 1, fields[4]))?;

        let ulo_name = fields[5].trim().to_string();
        let agent_name = fields[6].trim().to_string();

        configs.push(LicenseConfig {
            license_id,
            node_id: String::new(),
            lease_code,
            alias: String::new(),
            splits: LeaseSplit {
                uno_share,
                agent_share,
                ulo_share,
            },
            ulo_name,
            agent_name,
            uptime: 0.0,
            is_online: false,
            device_name: None,
            lease_from: None,
            lease_to: None,
        });
    }

    if configs.is_empty() {
        return Err("No valid license configurations found".to_string());
    }

    Ok(configs)
}

/// Parse a percentage string (e.g., "47%", "47.5", "47")
fn parse_percentage(s: &str) -> Result<f64, ()> {
    let s = s.trim_end_matches('%').trim();
    s.parse::<f64>().map_err(|_| ())
}

/// Build a lookup map from license_id to LicenseConfig
pub fn build_license_map(configs: Vec<LicenseConfig>) -> HashMap<String, LicenseConfig> {
    configs.into_iter().map(|c| (c.license_id.clone(), c)).collect()
}

/// Get unique agents from license configs
pub fn get_unique_agents(configs: &[LicenseConfig]) -> Vec<String> {
    let mut agents: Vec<String> = configs
        .iter()
        .filter(|c| !c.agent_name.is_empty())
        .map(|c| c.agent_name.clone())
        .collect();
    agents.sort();
    agents.dedup();
    agents
}

/// Get unique ULOs from license configs
pub fn get_unique_ulos(configs: &[LicenseConfig]) -> Vec<String> {
    let mut ulos: Vec<String> = configs
        .iter()
        .filter(|c| !c.ulo_name.is_empty())
        .map(|c| c.ulo_name.clone())
        .collect();
    ulos.sort();
    ulos.dedup();
    ulos
}

/// Get licenses for a specific agent
pub fn get_licenses_for_agent(configs: &[LicenseConfig], agent_name: &str) -> Vec<LicenseConfig> {
    configs
        .iter()
        .filter(|c| c.agent_name == agent_name)
        .cloned()
        .collect()
}

/// Get licenses for a specific ULO
pub fn get_licenses_for_ulo(configs: &[LicenseConfig], ulo_name: &str) -> Vec<LicenseConfig> {
    configs
        .iter()
        .filter(|c| c.ulo_name == ulo_name)
        .cloned()
        .collect()
}

/// Build node groupings from API licenses
pub fn build_nodes_from_api(api_licenses: &[LicenseApi]) -> Vec<Node> {
    let mut node_map: HashMap<String, Vec<&LicenseApi>> = HashMap::new();

    for license in api_licenses {
        node_map
            .entry(license.node_id.clone())
            .or_default()
            .push(license);
    }

    node_map
        .into_iter()
        .map(|(node_id, licenses)| {
            let online_count = licenses.iter().filter(|l| l.is_online).count();
            let total_count = licenses.len();
            let license_ids: Vec<String> = licenses.iter().map(|l| l.id.clone()).collect();
            Node {
                node_id,
                licenses: license_ids,
                total_earnings_micros: 0, // Will be computed separately
                online_count,
                total_count,
            }
        })
        .collect()
}

/// Merge API license data with local config
pub fn merge_license_with_config(
    api_license: &LicenseApi,
    local_config: Option<&LicenseConfig>,
) -> LicenseConfig {
    match local_config {
        Some(cfg) => {
            let mut merged = cfg.clone();
            merged.node_id = api_license.node_id.clone();
            merged.alias = api_license.alias.clone().unwrap_or_default();
            merged.uptime = api_license.uptime;
            merged.is_online = api_license.is_online;
            merged.device_name = api_license.device_name.clone();
            merged.lease_from = api_license.lease_from.clone();
            merged.lease_to = api_license.lease_to.clone();
            merged
        }
        None => {
            // Create config from API data only (no split info)
            LicenseConfig {
                license_id: api_license.id.clone(),
                node_id: api_license.node_id.clone(),
                lease_code: String::new(),
                alias: api_license.alias.clone().unwrap_or_default(),
                splits: crate::models::LeaseSplit::default(),
                ulo_name: String::new(),
                agent_name: String::new(),
                uptime: api_license.uptime,
                is_online: api_license.is_online,
                device_name: api_license.device_name.clone(),
                lease_from: api_license.lease_from.clone(),
                lease_to: api_license.lease_to.clone(),
            }
        }
    }
}

/// Merge all API licenses with local configs
pub fn merge_all_licenses(
    api_licenses: &[LicenseApi],
    local_configs: &HashMap<String, LicenseConfig>,
) -> Vec<LicenseConfig> {
    api_licenses
        .iter()
        .map(|api| merge_license_with_config(api, local_configs.get(&api.id)))
        .collect()
}

/// Calculate average uptime from analytics points
pub fn calculate_average_uptime(analytics: &[UptimeAnalyticsPoint]) -> f64 {
    if analytics.is_empty() {
        return 0.0;
    }
    let sum: f64 = analytics.iter().map(|p| p.uptime).sum();
    sum / analytics.len() as f64
}

/// Get license statistics
#[derive(Debug, Clone, Default)]
pub struct LicenseStats {
    pub total_licenses: usize,
    pub online_count: usize,
    pub offline_count: usize,
    pub avg_uptime: f64,
    pub leased_count: usize,
    pub active_agents: usize,
}

impl LicenseStats {
    pub fn from_configs(configs: &[LicenseConfig]) -> Self {
        let total_licenses = configs.len();
        let online_count = configs.iter().filter(|c| c.is_online).count();
        let offline_count = total_licenses - online_count;
        let avg_uptime = if configs.is_empty() {
            0.0
        } else {
            configs.iter().map(|c| c.uptime).sum::<f64>() / configs.len() as f64
        };
        let leased_count = configs.iter().filter(|c| !c.lease_code.is_empty()).count();
        let active_agents = get_unique_agents(configs).len();

        Self {
            total_licenses,
            online_count,
            offline_count,
            avg_uptime,
            leased_count,
            active_agents,
        }
    }

    pub fn from_api_licenses(licenses: &[LicenseApi]) -> Self {
        let total_licenses = licenses.len();
        let online_count = licenses.iter().filter(|l| l.is_online).count();
        let offline_count = total_licenses - online_count;
        let avg_uptime = if licenses.is_empty() {
            0.0
        } else {
            licenses.iter().map(|l| l.uptime).sum::<f64>() / licenses.len() as f64
        };
        let leased_count = licenses.iter().filter(|l| l.is_leased()).count();

        Self {
            total_licenses,
            online_count,
            offline_count,
            avg_uptime,
            leased_count,
            active_agents: 0, // Unknown without local config
        }
    }

    pub fn online_percentage(&self) -> f64 {
        if self.total_licenses == 0 {
            0.0
        } else {
            (self.online_count as f64 / self.total_licenses as f64) * 100.0
        }
    }

    pub fn avg_uptime_percentage(&self) -> f64 {
        self.avg_uptime * 100.0
    }
}
