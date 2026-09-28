//! Business logic for dashboard overview calculations

use crate::api::types::{RewardAllocation, LicenseApi};
use crate::models::LicenseConfig;
use crate::state::{
    TimeGranularity, DateRange, TopPerformer, GroupedEarnings, EarningsDistribution,
    LicensesOverviewData, AgentsOverviewData
};
use std::collections::HashMap;
use chrono::{NaiveDate, Datelike};

/// Calculate licenses overview data from API responses
pub fn calculate_licenses_overview(
    licenses: &[LicenseApi],
    allocations: &[RewardAllocation],
    local_configs: &HashMap<String, LicenseConfig>,
    balance_micros: i64,
    date_range: &DateRange,
    granularity: TimeGranularity,
) -> LicensesOverviewData {
    // Filter allocations by date range
    let filtered_allocations = filter_allocations_by_date(allocations, date_range);

    // Calculate total licenses and online count
    let total_licenses = licenses.len();
    let online_count = licenses.iter().filter(|l| l.is_online).count();
    let average_uptime = if licenses.is_empty() {
        0.0
    } else {
        licenses.iter().map(|l| l.uptime).sum::<f64>() / licenses.len() as f64 * 100.0
    };

    // Calculate total earnings (UNO share only - what API returns)
    let total_earnings = filtered_allocations
        .iter()
        .map(|a| a.amount_micros as f64 / 1_000_000.0)
        .sum();

    // Available earnings (unclaimed balance)
    let available_earnings = balance_micros as f64 / 1_000_000.0;

    // Calculate earnings per license
    let mut license_earnings: HashMap<String, f64> = HashMap::new();
    for alloc in &filtered_allocations {
        *license_earnings.entry(alloc.license_id.clone()).or_default() +=
            alloc.amount_micros as f64 / 1_000_000.0;
    }

    // Top performers (use alias if available)
    let top_performers = calculate_top_performers(&license_earnings, local_configs, 3);

    // Earnings distribution (for pie chart, use alias if available)
    let earnings_distribution = calculate_distribution(&license_earnings, local_configs);

    // Earnings over time (for bar chart)
    let earnings_over_time = group_by_period(&filtered_allocations, granularity);

    LicensesOverviewData {
        total_licenses,
        online_count,
        average_uptime,
        total_earnings,
        available_earnings,
        top_performers,
        earnings_distribution,
        earnings_over_time,
    }
}

/// Calculate agents overview data from DB configs first, then get their performance
pub fn calculate_agents_overview(
    allocations: &[RewardAllocation],
    local_configs: &HashMap<String, LicenseConfig>,
    date_range: &DateRange,
    granularity: TimeGranularity,
) -> AgentsOverviewData {
    // Step 1: Get all unique agents from DB (local_configs)
    let all_agents: Vec<String> = local_configs
        .values()
        .filter(|c| !c.agent_name.is_empty())
        .map(|c| c.agent_name.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let total_agents = all_agents.len();

    // Step 2: Build license -> agent mapping
    let mut license_to_agent: HashMap<String, (String, f64, f64)> = HashMap::new(); // license_id -> (agent_name, uno_share, agent_share)
    for config in local_configs.values() {
        if !config.agent_name.is_empty() {
            license_to_agent.insert(
                config.license_id.clone(),
                (config.agent_name.clone(), config.splits.uno_share, config.splits.agent_share)
            );
        }
    }

    // Step 3: Filter allocations by date range
    let filtered_allocations = filter_allocations_by_date(allocations, date_range);

    // Step 4: Calculate commissions for each agent
    let mut agent_commissions: HashMap<String, f64> = HashMap::new();
    let mut active_agents_set: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Initialize all agents with 0 commissions
    for agent in &all_agents {
        agent_commissions.insert(agent.clone(), 0.0);
    }

    // Calculate commissions from allocations
    for alloc in &filtered_allocations {
        if let Some((agent_name, uno_share, agent_share)) = license_to_agent.get(&alloc.license_id) {
            let total_share = uno_share + agent_share;
            if total_share > 0.0 {
                let commission = (alloc.amount_micros as f64 / 1_000_000.0) * (agent_share / total_share);
                *agent_commissions.entry(agent_name.clone()).or_default() += commission;
                if commission > 0.0 {
                    active_agents_set.insert(agent_name.clone());
                }
            }
        }
    }

    let active_agents = active_agents_set.len();
    let total_commissions: f64 = agent_commissions.values().sum();

    // Top performers (agents use their names directly)
    let top_performers = calculate_top_performers_by_name(&agent_commissions, 3);

    // Commissions distribution
    let commissions_distribution = calculate_distribution_by_name(&agent_commissions);

    // Commissions over time (for bar chart)
    let commissions_over_time = group_agent_commissions_by_period(
        &filtered_allocations,
        local_configs,
        granularity,
    );

    AgentsOverviewData {
        total_agents,
        active_agents,
        total_commissions,
        available_commissions: 0.0,
        top_performers,
        commissions_distribution,
        commissions_over_time,
    }
}

/// Filter allocations by date range
fn filter_allocations_by_date(allocations: &[RewardAllocation], date_range: &DateRange) -> Vec<RewardAllocation> {
    allocations
        .iter()
        .filter(|a| {
            // Parse allocation date - completed_at is "YYYY-MM-DDTHH:MM:SS..."
            let date_str = a.completed_at.split('T').next().unwrap_or("");
            date_str >= date_range.start.as_str() && date_str <= date_range.end.as_str()
        })
        .cloned()
        .collect()
}

/// Calculate top performers from earnings map
fn calculate_top_performers(
    earnings: &HashMap<String, f64>,
    local_configs: &HashMap<String, LicenseConfig>,
    limit: usize,
) -> Vec<TopPerformer> {
    let mut sorted: Vec<_> = earnings.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    sorted
        .into_iter()
        .take(limit)
        .enumerate()
        .map(|(i, (id, &amount))| TopPerformer {
            id: id.clone(),
            name: get_display_name(id, local_configs),
            earnings: amount,
            rank: (i + 1) as u8,
        })
        .collect()
}

/// Get display name: use alias if present, otherwise truncate license ID
fn get_display_name(id: &str, local_configs: &HashMap<String, LicenseConfig>) -> String {
    // Check if there's an alias in local config
    if let Some(config) = local_configs.get(id) {
        if !config.alias.is_empty() {
            return config.alias.clone();
        }
    }
    // Fall back to truncated ID
    truncate_license_id(id)
}

/// Truncate license ID for display
fn truncate_license_id(id: &str) -> String {
    if id.len() > 12 {
        format!("{}...{}", &id[..6], &id[id.len()-4..])
    } else {
        id.to_string()
    }
}

/// Calculate top performers by name (for agents - names are already human-readable)
fn calculate_top_performers_by_name(earnings: &HashMap<String, f64>, limit: usize) -> Vec<TopPerformer> {
    let mut sorted: Vec<_> = earnings.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    sorted
        .into_iter()
        .take(limit)
        .enumerate()
        .map(|(i, (name, &amount))| TopPerformer {
            id: name.clone(),
            name: name.clone(),
            earnings: amount,
            rank: (i + 1) as u8,
        })
        .collect()
}

/// Calculate distribution by name (for agents - names are already human-readable)
fn calculate_distribution_by_name(earnings: &HashMap<String, f64>) -> Vec<EarningsDistribution> {
    let total: f64 = earnings.values().sum();
    if total == 0.0 {
        return vec![];
    }

    let mut sorted: Vec<_> = earnings.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Take top 7 and group rest as "Other"
    let (top, rest): (Vec<_>, Vec<_>) = sorted.into_iter().enumerate().partition(|(i, _)| *i < 7);

    let mut distribution: Vec<EarningsDistribution> = top
        .into_iter()
        .map(|(_, (name, &amount))| EarningsDistribution {
            id: name.clone(),
            name: name.clone(),
            amount,
            percentage: (amount / total) * 100.0,
        })
        .collect();

    // Add "Other" if there are more items
    if !rest.is_empty() {
        let other_amount: f64 = rest.iter().map(|(_, (_, &v))| v).sum();
        if other_amount > 0.0 {
            distribution.push(EarningsDistribution {
                id: "other".to_string(),
                name: "Other".to_string(),
                amount: other_amount,
                percentage: (other_amount / total) * 100.0,
            });
        }
    }

    distribution
}

/// Calculate earnings distribution for pie chart
fn calculate_distribution(
    earnings: &HashMap<String, f64>,
    local_configs: &HashMap<String, LicenseConfig>,
) -> Vec<EarningsDistribution> {
    let total: f64 = earnings.values().sum();
    if total == 0.0 {
        return vec![];
    }

    let mut sorted: Vec<_> = earnings.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Take top 7 and group rest as "Other"
    let (top, rest): (Vec<_>, Vec<_>) = sorted.into_iter().enumerate().partition(|(i, _)| *i < 7);

    let mut distribution: Vec<EarningsDistribution> = top
        .into_iter()
        .map(|(_, (id, &amount))| EarningsDistribution {
            id: id.clone(),
            name: get_display_name(id, local_configs),
            amount,
            percentage: (amount / total) * 100.0,
        })
        .collect();

    // Add "Other" if there are more items
    if !rest.is_empty() {
        let other_amount: f64 = rest.iter().map(|(_, (_, &v))| v).sum();
        if other_amount > 0.0 {
            distribution.push(EarningsDistribution {
                id: "other".to_string(),
                name: "Other".to_string(),
                amount: other_amount,
                percentage: (other_amount / total) * 100.0,
            });
        }
    }

    distribution
}

/// Group allocations by time period
fn group_by_period(allocations: &[RewardAllocation], granularity: TimeGranularity) -> Vec<GroupedEarnings> {
    let mut grouped: HashMap<String, f64> = HashMap::new();

    for alloc in allocations {
        let date_str = alloc.completed_at.split('T').next().unwrap_or("");
        if !date_str.is_empty() {
            let period_key = get_period_key(date_str, granularity);
            *grouped.entry(period_key).or_default() += alloc.amount_micros as f64 / 1_000_000.0;
        }
    }

    let mut result: Vec<_> = grouped
        .into_iter()
        .map(|(period, amount)| GroupedEarnings { period, amount })
        .collect();

    // Sort by period
    result.sort_by(|a, b| a.period.cmp(&b.period));

    result
}

/// Group agent commissions by time period
fn group_agent_commissions_by_period(
    allocations: &[RewardAllocation],
    local_configs: &HashMap<String, LicenseConfig>,
    granularity: TimeGranularity,
) -> Vec<GroupedEarnings> {
    let mut grouped: HashMap<String, f64> = HashMap::new();

    for alloc in allocations {
        if let Some(config) = local_configs.get(&alloc.license_id) {
            let uno_share = config.splits.uno_share;
            let agent_share = config.splits.agent_share;
            let total_share = uno_share + agent_share;

            if total_share > 0.0 && !config.agent_name.is_empty() {
                let commission = (alloc.amount_micros as f64 / 1_000_000.0) * (agent_share / total_share);
                let date_str = alloc.completed_at.split('T').next().unwrap_or("");
                if !date_str.is_empty() {
                    let period_key = get_period_key(date_str, granularity);
                    *grouped.entry(period_key).or_default() += commission;
                }
            }
        }
    }

    let mut result: Vec<_> = grouped
        .into_iter()
        .map(|(period, amount)| GroupedEarnings { period, amount })
        .collect();

    result.sort_by(|a, b| a.period.cmp(&b.period));

    result
}

/// Get period key based on granularity
fn get_period_key(date_str: &str, granularity: TimeGranularity) -> String {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => match granularity {
            TimeGranularity::Daily => date.format("%m/%d").to_string(),
            TimeGranularity::Weekly => {
                let week = date.iso_week().week();
                format!("W{}", week)
            }
            TimeGranularity::Monthly => date.format("%b").to_string(),
            TimeGranularity::Quarterly => {
                let quarter = (date.month() - 1) / 3 + 1;
                format!("Q{}", quarter)
            }
            TimeGranularity::Yearly => date.format("%Y").to_string(),
        },
        Err(_) => date_str.to_string(),
    }
}
