//! Business logic for agent and ULO operations

use crate::api::types::RewardAllocation;
use crate::models::{AgentPerformance, ComputedAllocation, LicenseConfig};
use std::collections::HashMap;

/// Calculate agent performance from allocations and license configs
pub fn calculate_agent_performance(
    allocations: &[RewardAllocation],
    license_configs: &HashMap<String, LicenseConfig>,
) -> Vec<AgentPerformance> {
    // Group allocations by agent
    let mut agent_allocations: HashMap<String, Vec<&RewardAllocation>> = HashMap::new();

    for allocation in allocations {
        if let Some(config) = license_configs.get(&allocation.license_id) {
            if !config.agent_name.is_empty() {
                agent_allocations
                    .entry(config.agent_name.clone())
                    .or_default()
                    .push(allocation);
            }
        }
    }

    // Calculate performance for each agent
    agent_allocations
        .into_iter()
        .map(|(agent_name, allocs)| {
            // Get unique ULOs and licenses for this agent
            let mut ulo_names: Vec<String> = Vec::new();
            let mut license_ids: Vec<String> = Vec::new();
            let mut total_earnings_micros: i64 = 0;
            let mut agent_share_micros: i64 = 0;
            let mut uptime_sum: f64 = 0.0;

            for allocation in &allocs {
                if let Some(config) = license_configs.get(&allocation.license_id) {
                    // Track unique ULOs
                    if !ulo_names.contains(&config.ulo_name) && !config.ulo_name.is_empty() {
                        ulo_names.push(config.ulo_name.clone());
                    }
                    // Track unique licenses
                    if !license_ids.contains(&allocation.license_id) {
                        license_ids.push(allocation.license_id.clone());
                        uptime_sum += config.uptime;
                    }

                    // Calculate shares
                    let computed = ComputedAllocation::compute((*allocation).clone(), Some(config));
                    total_earnings_micros += allocation.amount_micros;
                    agent_share_micros += computed.agent_share_micros;
                }
            }

            let license_count = license_ids.len();
            let avg_uptime = if license_count > 0 {
                uptime_sum / license_count as f64
            } else {
                0.0
            };

            // Count online licenses for this agent
            let online_licenses = license_ids.iter()
                .filter(|lid| license_configs.get(*lid).map(|c| c.is_online).unwrap_or(false))
                .count();

            AgentPerformance {
                agent_name,
                ulo_count: ulo_names.len(),
                license_count,
                total_earnings_micros,
                agent_share_micros,
                allocations_count: allocs.len(),
                avg_uptime,
                online_licenses,
            }
        })
        .collect()
}

/// Sort agent performance by total earnings (descending)
pub fn sort_agents_by_earnings(mut agents: Vec<AgentPerformance>) -> Vec<AgentPerformance> {
    agents.sort_by(|a, b| b.total_earnings_micros.cmp(&a.total_earnings_micros));
    agents
}

/// Sort agent performance by commission (descending)
pub fn sort_agents_by_commission(mut agents: Vec<AgentPerformance>) -> Vec<AgentPerformance> {
    agents.sort_by(|a, b| b.agent_share_micros.cmp(&a.agent_share_micros));
    agents
}

/// Sort agent performance by license count (descending)
pub fn sort_agents_by_licenses(mut agents: Vec<AgentPerformance>) -> Vec<AgentPerformance> {
    agents.sort_by(|a, b| b.license_count.cmp(&a.license_count));
    agents
}

/// Get top N agents by commission
pub fn get_top_agents(agents: Vec<AgentPerformance>, limit: usize) -> Vec<AgentPerformance> {
    let sorted = sort_agents_by_commission(agents);
    sorted.into_iter().take(limit).collect()
}

/// ULO performance summary
#[derive(Debug, Clone, Default)]
pub struct UloPerformance {
    pub ulo_name: String,
    pub agent_name: String,
    pub license_count: usize,
    pub total_earnings_micros: i64,
    pub avg_uptime: f64,
}

/// Calculate ULO performance from allocations and license configs
pub fn calculate_ulo_performance(
    allocations: &[RewardAllocation],
    license_configs: &HashMap<String, LicenseConfig>,
) -> Vec<UloPerformance> {
    // Group allocations by ULO
    let mut ulo_allocations: HashMap<String, Vec<&RewardAllocation>> = HashMap::new();

    for allocation in allocations {
        if let Some(config) = license_configs.get(&allocation.license_id) {
            if !config.ulo_name.is_empty() {
                ulo_allocations
                    .entry(config.ulo_name.clone())
                    .or_default()
                    .push(allocation);
            }
        }
    }

    // Calculate performance for each ULO
    ulo_allocations
        .into_iter()
        .map(|(ulo_name, allocs)| {
            let mut license_ids: Vec<String> = Vec::new();
            let mut total_earnings_micros: i64 = 0;
            let mut uptime_sum: f64 = 0.0;
            let mut agent_name = String::new();

            for allocation in &allocs {
                if let Some(config) = license_configs.get(&allocation.license_id) {
                    // Track unique licenses
                    if !license_ids.contains(&allocation.license_id) {
                        license_ids.push(allocation.license_id.clone());
                        uptime_sum += config.uptime;
                        agent_name = config.agent_name.clone();
                    }
                    total_earnings_micros += allocation.amount_micros;
                }
            }

            let license_count = license_ids.len();
            let avg_uptime = if license_count > 0 {
                uptime_sum / license_count as f64
            } else {
                0.0
            };

            UloPerformance {
                ulo_name,
                agent_name,
                license_count,
                total_earnings_micros,
                avg_uptime,
            }
        })
        .collect()
}

/// Calculate agent's ULOs from license configs
pub fn get_agent_ulos(
    agent_name: &str,
    license_configs: &HashMap<String, LicenseConfig>,
) -> Vec<String> {
    let mut ulos: Vec<String> = license_configs
        .values()
        .filter(|c| c.agent_name == agent_name && !c.ulo_name.is_empty())
        .map(|c| c.ulo_name.clone())
        .collect();
    ulos.sort();
    ulos.dedup();
    ulos
}

/// Calculate allocations summary for a specific agent
pub fn get_agent_allocations_summary(
    agent_name: &str,
    allocations: &[RewardAllocation],
    license_configs: &HashMap<String, LicenseConfig>,
) -> (i64, i64, usize) {
    // Returns (total_earnings_micros, agent_share_micros, allocation_count)
    let mut total_earnings: i64 = 0;
    let mut agent_share: i64 = 0;
    let mut count: usize = 0;

    for allocation in allocations {
        if let Some(config) = license_configs.get(&allocation.license_id) {
            if config.agent_name == agent_name {
                let computed = ComputedAllocation::compute(allocation.clone(), Some(config));
                total_earnings += allocation.amount_micros;
                agent_share += computed.agent_share_micros;
                count += 1;
            }
        }
    }

    (total_earnings, agent_share, count)
}
