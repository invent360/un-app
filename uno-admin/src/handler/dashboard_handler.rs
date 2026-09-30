//! Dashboard overview handlers that fetch data from DB

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::state::{
    DateRange, TimeGranularity, AgentsOverviewData, TopPerformer,
    EarningsDistribution, GroupedEarnings
};
use crate::models::entity::{AgentEntity, LicenseEntity};
use crate::api::types::RewardAllocation;
use std::collections::HashMap;

/// Agent info with their license IDs for commission calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentWithLicenses {
    pub agent: AgentEntity,
    pub license_ids: Vec<String>,
    pub uno_share: f64,
    pub agent_share: f64,
}

/// Server function to get agents overview data from database
#[server(GetAgentsOverviewFromDb, "/api")]
pub async fn get_agents_overview_from_db() -> Result<Vec<AgentWithLicenses>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;
    use crate::repository::traits::LicenseRepositoryTrait;

    use crate::repository::postgres::PgLicenseRepository as LicenseRepository;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    // Fetch all agents
    let agent_service = AgentService::new(pool.clone());
    let agents = agent_service
        .list_agents()
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Fetch all licenses to get their agent_id mappings
    let license_repo = LicenseRepository::new(pool);
    let licenses = license_repo
        .list_licenses()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Build agent -> licenses mapping
    let mut agent_licenses: HashMap<String, Vec<LicenseEntity>> = HashMap::new();
    for license in licenses {
        agent_licenses
            .entry(license.agent_id.clone())
            .or_default()
            .push(license);
    }

    // Create AgentWithLicenses for each agent
    let result: Vec<AgentWithLicenses> = agents
        .into_iter()
        .map(|agent| {
            let licenses = agent_licenses.get(&agent.id).cloned().unwrap_or_default();

            // Get shares from first license or use defaults
            let (uno_share, agent_share) = if let Some(first_license) = licenses.first() {
                (first_license.uno_share, first_license.agent_share)
            } else {
                (47.0, 3.0) // Default shares
            };

            let license_ids: Vec<String> = licenses.iter().map(|l| l.license_id.clone()).collect();

            AgentWithLicenses {
                agent,
                license_ids,
                uno_share,
                agent_share,
            }
        })
        .collect();

    Ok(result)
}

/// Server function to get all rewards from database (for dashboard charts)
#[server(GetRewardsFromDb, "/api")]
pub async fn get_rewards_from_db() -> Result<Vec<RewardAllocation>, ServerFnError> {
    use crate::db::get_db;
    use crate::repository::traits::RewardRepositoryTrait;
    use crate::models::entity::PaginationParams;

    use crate::repository::postgres::PgRewardRepository as RewardRepository;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let reward_repo = RewardRepository::new(pool);

    // Get up to 1000 rewards for dashboard
    let pagination = PaginationParams { page: 0, limit: 1000 };
    let paginated_result = reward_repo
        .list_rewards(pagination)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Convert RewardEntity to RewardAllocation
    let allocations: Vec<RewardAllocation> = paginated_result.items
        .into_iter()
        .map(|r| RewardAllocation {
            id: r.id,
            user_id: r.user_id,
            license_id: r.license_id,
            license_lease_id: r.license_lease_id,
            node_id: r.node_id,
            task_key: r.task_key,
            task_metadata: r.task_metadata.map(|s| serde_json::Value::String(s)),
            allocation_type: r.reward_type,
            description: r.description,
            amount_micros: r.amount_micros,
            completed_at: r.completed_at,
            created_at: r.created_at,
        })
        .collect();

    Ok(allocations)
}

/// Calculate agents overview data from DB agents and API allocations
pub fn calculate_agents_overview_from_db(
    db_agents: &[AgentWithLicenses],
    allocations: &[RewardAllocation],
    date_range: &DateRange,
    granularity: TimeGranularity,
) -> AgentsOverviewData {
    use chrono::{NaiveDate, Datelike};

    // Filter allocations by date range
    let filtered_allocations: Vec<_> = allocations
        .iter()
        .filter(|a| {
            let date_str = a.completed_at.split('T').next().unwrap_or("");
            date_str >= date_range.start.as_str() && date_str <= date_range.end.as_str()
        })
        .collect();

    let total_agents = db_agents.len();

    // Build license_id -> (agent_name, uno_share, agent_share) mapping
    let mut license_to_agent: HashMap<String, (String, f64, f64)> = HashMap::new();
    for agent_data in db_agents {
        for license_id in &agent_data.license_ids {
            license_to_agent.insert(
                license_id.clone(),
                (agent_data.agent.name.clone(), agent_data.uno_share, agent_data.agent_share)
            );
        }
    }

    // Calculate commissions for each agent
    let mut agent_commissions: HashMap<String, f64> = HashMap::new();
    let mut active_agents_set: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Initialize all agents with 0 commissions
    for agent_data in db_agents {
        agent_commissions.insert(agent_data.agent.name.clone(), 0.0);
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

    // Top performers
    let top_performers = calculate_top_performers_by_name(&agent_commissions, 3);

    // Commissions distribution
    let commissions_distribution = calculate_distribution_by_name(&agent_commissions);

    // Commissions over time
    let commissions_over_time = group_agent_commissions_by_period_from_db(
        &filtered_allocations,
        &license_to_agent,
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

/// Calculate top performers by name
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

/// Calculate distribution by name
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

/// Group agent commissions by time period from DB data
fn group_agent_commissions_by_period_from_db(
    allocations: &[&RewardAllocation],
    license_to_agent: &HashMap<String, (String, f64, f64)>,
    granularity: TimeGranularity,
) -> Vec<GroupedEarnings> {
    use chrono::{NaiveDate, Datelike};

    let mut grouped: HashMap<String, f64> = HashMap::new();

    for alloc in allocations {
        if let Some((_, uno_share, agent_share)) = license_to_agent.get(&alloc.license_id) {
            let total_share = uno_share + agent_share;

            if total_share > 0.0 {
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
    use chrono::{NaiveDate, Datelike};

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
