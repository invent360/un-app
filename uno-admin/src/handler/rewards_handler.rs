//! Handlers for rewards operations with reactive state
//! NOTE: Rewards logic is temporarily disabled - depends on removed client module

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::types::*;
// use crate::logic::*;  // Temporarily disabled - depends on removed client module
use crate::models::{ComputedAllocation, LicenseConfig, RevenueSummary};
use std::collections::HashMap;

/// Stub type for node allocations (original in disabled rewards_logic module)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeAllocations {
    pub node_id: String,
    pub allocations: Vec<RewardAllocation>,
    pub total_amount_micros: i64,
    pub allocation_count: usize,
}

/// State for the rewards page
#[derive(Debug, Clone)]
pub struct RewardsState {
    pub allocations: Vec<RewardAllocation>,
    pub computed_allocations: Vec<ComputedAllocation>,
    pub summary: AllocationsSummary,
    pub api_summary: AllocationsSummaryApi,
    pub balance: i64,
    pub current_page: u32,
    pub page_size: u32,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for RewardsState {
    fn default() -> Self {
        Self {
            allocations: vec![],
            computed_allocations: vec![],
            summary: AllocationsSummary::default(),
            api_summary: AllocationsSummaryApi::default(),
            balance: 0,
            current_page: 0,
            page_size: 20,
            loading: false,
            error: None,
        }
    }
}

impl RewardsState {
    /// Get allocations for the current page
    pub fn current_page_allocations(&self) -> &[RewardAllocation] {
        let start = (self.current_page * self.page_size) as usize;
        let end = std::cmp::min(start + self.page_size as usize, self.allocations.len());
        if start >= self.allocations.len() {
            &[]
        } else {
            &self.allocations[start..end]
        }
    }

    /// Get computed allocations for the current page
    pub fn current_page_computed(&self) -> Vec<&ComputedAllocation> {
        let start = (self.current_page * self.page_size) as usize;
        let end = std::cmp::min(start + self.page_size as usize, self.computed_allocations.len());
        if start >= self.computed_allocations.len() {
            vec![]
        } else {
            self.computed_allocations[start..end].iter().collect()
        }
    }

    /// Total number of pages
    pub fn total_pages(&self) -> u32 {
        if self.allocations.is_empty() {
            0
        } else {
            ((self.allocations.len() as u32 - 1) / self.page_size) + 1
        }
    }

    /// Check if there's a next page
    pub fn has_next_page(&self) -> bool {
        self.current_page + 1 < self.total_pages()
    }

    /// Check if there's a previous page
    pub fn has_prev_page(&self) -> bool {
        self.current_page > 0
    }

    /// Compute revenue summary
    pub fn revenue_summary(&self) -> RevenueSummary {
        use std::collections::HashSet;

        let total_original_micros: i64 = self.allocations.iter().map(|a| a.amount_micros).sum();
        let total_uno_share_micros: i64 = self.computed_allocations.iter().map(|c| c.uno_share_micros).sum();
        let total_agent_share_micros: i64 = self.computed_allocations.iter().map(|c| c.agent_share_micros).sum();

        // Count unique entities
        let unique_licenses: HashSet<_> = self.allocations.iter().map(|a| &a.license_id).collect();
        let unique_nodes: HashSet<_> = self.allocations.iter().map(|a| &a.node_id).collect();
        let unique_agents: HashSet<_> = self.computed_allocations
            .iter()
            .filter_map(|c| c.agent_name.as_ref())
            .collect();

        RevenueSummary {
            total_allocations: self.allocations.len(),
            total_original_micros,
            total_uno_share_micros,
            total_agent_share_micros,
            unique_licenses: unique_licenses.len(),
            unique_agents: unique_agents.len(),
            unique_nodes: unique_nodes.len(),
            unclaimed_balance_micros: self.balance,
            today_amount_micros: self.api_summary.today_amount_micros,
            this_week_amount_micros: self.api_summary.this_week_amount_micros,
            last_7_days_amount_micros: self.api_summary.last7_days_amount_micros,
        }
    }
}

/// Handler for fetching all rewards allocations
/// NOTE: Temporarily stubbed - rewards logic depends on removed client module
pub async fn fetch_all_allocations_handler(
    _token: String,
) -> Result<(Vec<RewardAllocation>, AllocationsSummary), String> {
    Ok((vec![], AllocationsSummary::default()))
}

/// Handler for fetching a single page of allocations
/// NOTE: Temporarily stubbed - rewards logic depends on removed client module
pub async fn fetch_allocations_page_handler(
    _token: String,
    page: u32,
    page_size: u32,
) -> Result<PaginatedResponse<RewardAllocation>, String> {
    Ok(PaginatedResponse::new(vec![], page, page_size))
}

/// Handler for fetching allocations grouped by node
/// NOTE: Temporarily stubbed - rewards logic depends on removed client module
pub async fn fetch_allocations_by_node_handler(
    _token: String,
) -> Result<Vec<NodeAllocations>, String> {
    Ok(vec![])
}

/// Handler for fetching balance and summary
/// NOTE: Temporarily stubbed - rewards logic depends on removed client module
pub async fn fetch_rewards_dashboard_handler(
    _token: String,
) -> Result<(i64, AllocationsSummaryApi), String> {
    Ok((0, AllocationsSummaryApi::default()))
}

/// Result of a sync operation (serializable for client)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncResultDto {
    /// Whether the data was stale before sync
    pub was_stale: bool,
    /// Number of new rewards synced from API
    pub rewards_synced: usize,
    /// True if no sync was needed (data was fresh)
    pub already_up_to_date: bool,
}

/// Check and sync rewards from API if database data is stale
/// Syncs yesterday's rewards if no completed job exists, and today's rewards for real-time data
#[server(SyncRewardsIfStale, "/api")]
pub async fn sync_rewards_if_stale(_jwt_token: String) -> Result<SyncResultDto, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::RewardsSyncService;
    use crate::repository::traits::SyncJobRepositoryTrait;
    use chrono::{Utc, Duration};

    use crate::repository::postgres::PgSyncJobRepository as SyncJobRepository;

    println!("[SYNC] Checking if rewards data is stale...");

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let job_repo = SyncJobRepository::new(pool.clone());
    let service = RewardsSyncService::new(pool);

    // Calculate target dates
    let now = Utc::now();
    let today = now.format("%Y-%m-%d").to_string();
    let yesterday = (now - Duration::days(1)).format("%Y-%m-%d").to_string();

    let mut total_synced: usize = 0;
    let mut was_stale = false;

    // Check and sync yesterday's rewards
    let yesterday_job = job_repo
        .get_job_by_date("rewards_sync", &yesterday)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to check sync job: {}", e)))?;

    let yesterday_needs_sync = match &yesterday_job {
        Some(job) => job.status != "completed",
        None => true,
    };

    if yesterday_needs_sync {
        println!("[SYNC] Yesterday ({}) needs sync", yesterday);
        was_stale = true;
        match service.sync_rewards_for_date(&yesterday).await {
            Ok(job) => {
                println!("[SYNC] Yesterday sync completed: {} records inserted", job.records_inserted);
                total_synced += job.records_inserted as usize;
            }
            Err(e) => {
                println!("[SYNC] Yesterday sync failed: {}", e);
                // Continue to try today's sync
            }
        }
    }

    // Always try to sync today's rewards for real-time data
    let today_job = job_repo
        .get_job_by_date("rewards_sync", &today)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to check sync job: {}", e)))?;

    let today_needs_sync = match &today_job {
        Some(job) => job.status != "completed" && job.status != "running",
        None => true,
    };

    if today_needs_sync {
        println!("[SYNC] Today ({}) needs sync", today);
        was_stale = true;
        match service.sync_rewards_for_date(&today).await {
            Ok(job) => {
                println!("[SYNC] Today sync completed: {} records inserted", job.records_inserted);
                total_synced += job.records_inserted as usize;
            }
            Err(e) => {
                // Today's rewards may not be available yet - this is expected
                println!("[SYNC] Today sync skipped: {}", e);
            }
        }
    }

    if !was_stale {
        println!("[SYNC] Data is already up to date");
    } else {
        println!("[SYNC] Sync complete. Total records synced: {}", total_synced);
    }

    Ok(SyncResultDto {
        was_stale,
        rewards_synced: total_synced,
        already_up_to_date: !was_stale,
    })
}

/// License rewards summary from database
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LicenseRewardsSummaryDto {
    pub license_id: String,
    pub total_amount_micros: i64,
    pub last7_days_amount_micros: i64,
    pub this_week_amount_micros: i64,
    pub today_amount_micros: i64,
    pub reward_count: i64,
}

/// Get rewards summary for a single license from database
#[server(GetLicenseRewardsSummary, "/api")]
pub async fn get_license_rewards_summary(license_id: String) -> Result<LicenseRewardsSummaryDto, ServerFnError> {
    use crate::db::get_db;
    use crate::repository::RewardRepository;
    use crate::repository::traits::RewardRepositoryTrait;
    use crate::models::entity::{ListRewardsByLicenseAndDateParams, PaginationParams};
    use chrono::{Utc, Duration, Datelike};

    println!("[REWARDS] Fetching rewards summary for license: {}", license_id);

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?;

    let repo = RewardRepository::new(pool);

    // Get total earnings
    let total_amount_micros = repo
        .get_total_earnings_by_license_id(&license_id)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Get reward count
    let reward_count = repo
        .count_rewards_by_license_id(&license_id)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Calculate date ranges
    let now = Utc::now();
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let seven_days_ago = now - Duration::days(7);

    // This week (Monday start)
    let days_since_monday = now.weekday().num_days_from_monday() as i64;
    let week_start = (now - Duration::days(days_since_monday)).date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

    // Get last 7 days rewards
    let last7_params = ListRewardsByLicenseAndDateParams {
        license_id: license_id.clone(),
        start_date: seven_days_ago.to_rfc3339(),
        end_date: now.to_rfc3339(),
        pagination: PaginationParams { page: 0, limit: 10000 },
    };
    let last7_result = repo
        .list_rewards_by_license_id_and_date(last7_params)
        .await
        .map_err(|e| ServerFnError::new(e))?;
    let last7_days_amount_micros: i64 = last7_result.items.iter().map(|r| r.amount_micros).sum();

    // Get this week rewards
    let week_params = ListRewardsByLicenseAndDateParams {
        license_id: license_id.clone(),
        start_date: week_start.to_rfc3339(),
        end_date: now.to_rfc3339(),
        pagination: PaginationParams { page: 0, limit: 10000 },
    };
    let week_result = repo
        .list_rewards_by_license_id_and_date(week_params)
        .await
        .map_err(|e| ServerFnError::new(e))?;
    let this_week_amount_micros: i64 = week_result.items.iter().map(|r| r.amount_micros).sum();

    // Get today rewards
    let today_params = ListRewardsByLicenseAndDateParams {
        license_id: license_id.clone(),
        start_date: today_start.to_rfc3339(),
        end_date: now.to_rfc3339(),
        pagination: PaginationParams { page: 0, limit: 10000 },
    };
    let today_result = repo
        .list_rewards_by_license_id_and_date(today_params)
        .await
        .map_err(|e| ServerFnError::new(e))?;
    let today_amount_micros: i64 = today_result.items.iter().map(|r| r.amount_micros).sum();

    println!("[REWARDS] License {} - total: {}, last7: {}, week: {}, today: {}",
             license_id, total_amount_micros, last7_days_amount_micros, this_week_amount_micros, today_amount_micros);

    Ok(LicenseRewardsSummaryDto {
        license_id,
        total_amount_micros,
        last7_days_amount_micros,
        this_week_amount_micros,
        today_amount_micros,
        reward_count,
    })
}

/// Get rewards summary for multiple licenses from database (batch operation)
#[server(GetLicensesRewardsSummary, "/api")]
pub async fn get_licenses_rewards_summary(license_ids: Vec<String>) -> Result<Vec<LicenseRewardsSummaryDto>, ServerFnError> {
    println!("[REWARDS] Fetching rewards summary for {} licenses", license_ids.len());

    let mut results = Vec::new();
    for license_id in license_ids {
        match get_license_rewards_summary(license_id.clone()).await {
            Ok(summary) => results.push(summary),
            Err(e) => {
                println!("[REWARDS] Error fetching summary for {}: {}", license_id, e);
                // Return empty summary for this license
                results.push(LicenseRewardsSummaryDto {
                    license_id,
                    ..Default::default()
                });
            }
        }
    }

    println!("[REWARDS] Completed fetching {} license summaries", results.len());
    Ok(results)
}

/// Daily reward data for charts
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DailyRewardDto {
    pub date: String,
    pub total_amount_micros: i64,
    pub max_amount_micros: i64,
    pub count: i64,
}

/// Get daily rewards for a license from database (for charts)
/// Returns last 30 days of data
#[server(GetLicenseDailyRewards, "/api")]
pub async fn get_license_daily_rewards(license_id: String) -> Result<Vec<DailyRewardDto>, ServerFnError> {
    use crate::db::get_db;
    use crate::repository::RewardRepository;
    use crate::repository::traits::RewardRepositoryTrait;
    use crate::models::entity::{ListRewardsByLicenseParams, PaginationParams};
    use std::collections::BTreeMap;
    use chrono::{Utc, Duration};

    println!("[REWARDS] Fetching daily rewards for license: {}", license_id);

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let repo = RewardRepository::new(pool);

    // Get all rewards for this license (use large limit to get all)
    let params = ListRewardsByLicenseParams {
        license_id: license_id.clone(),
        pagination: PaginationParams { page: 0, limit: 10000 },
    };
    let rewards_result = repo
        .list_rewards_by_license_id(params)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    println!("[REWARDS] Found {} rewards for license {}", rewards_result.items.len(), license_id);

    // Calculate cutoff date (30 days ago)
    let thirty_days_ago = (Utc::now() - Duration::days(30)).format("%Y-%m-%d").to_string();

    // Group by date, filtering to last 30 days
    let mut daily_data: BTreeMap<String, (i64, i64, i64)> = BTreeMap::new(); // (total, max, count)

    for reward in rewards_result.items {
        // Extract date from completed_at (format: "2025-01-15T12:00:00Z" -> "2025-01-15")
        let date = reward.completed_at
            .split('T')
            .next()
            .unwrap_or(&reward.completed_at)
            .to_string();

        // Skip if older than 30 days
        if date < thirty_days_ago {
            continue;
        }

        let entry = daily_data.entry(date).or_insert((0, 0, 0));
        entry.0 += reward.amount_micros; // total
        entry.1 = entry.1.max(reward.amount_micros); // max
        entry.2 += 1; // count
    }

    // Convert to DTOs
    let result: Vec<DailyRewardDto> = daily_data
        .into_iter()
        .map(|(date, (total, max, count))| DailyRewardDto {
            date,
            total_amount_micros: total,
            max_amount_micros: max,
            count,
        })
        .collect();

    println!("[REWARDS] Returning {} daily data points (last 30 days)", result.len());
    Ok(result)
}

/// Get daily rewards for an agent (aggregates all licenses belonging to the agent)
/// Returns last 7 days of data for charts
#[server(GetAgentDailyRewards, "/api")]
pub async fn get_agent_daily_rewards(agent_id: String) -> Result<Vec<DailyRewardDto>, ServerFnError> {
    use crate::db::get_db;
    use crate::repository::{LicenseRepository, RewardRepository};
    use crate::repository::traits::{LicenseRepositoryTrait, RewardRepositoryTrait};
    use crate::models::entity::{ListRewardsByLicenseParams, PaginationParams};
    use std::collections::BTreeMap;
    use chrono::{Utc, Duration};

    println!("[REWARDS] Fetching daily rewards for agent: {}", agent_id);

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    // Get licenses for this agent
    let license_repo = LicenseRepository::new(pool.clone());
    let licenses = license_repo
        .get_licenses_by_agent_id(&agent_id)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    let license_ids: Vec<String> = licenses.iter().map(|l| l.license_id.clone()).collect();
    println!("[REWARDS] Agent has {} licenses", license_ids.len());

    if license_ids.is_empty() {
        return Ok(vec![]);
    }

    // Calculate date range (last 30 days)
    let now = Utc::now();
    let thirty_days_ago = (now - Duration::days(30)).format("%Y-%m-%d").to_string();

    // Aggregate rewards from all licenses by date
    let reward_repo = RewardRepository::new(pool);
    let mut daily_data: BTreeMap<String, (i64, i64, i64)> = BTreeMap::new(); // (total, max, count)

    for license_id in license_ids {
        let params = ListRewardsByLicenseParams {
            license_id: license_id.clone(),
            pagination: PaginationParams { page: 0, limit: 10000 },
        };

        match reward_repo.list_rewards_by_license_id(params).await {
            Ok(rewards_result) => {
                for reward in rewards_result.items {
                    // Extract date from completed_at
                    let date = reward.completed_at
                        .split('T')
                        .next()
                        .unwrap_or(&reward.completed_at)
                        .to_string();

                    // Only include last 30 days
                    if date >= thirty_days_ago {
                        let entry = daily_data.entry(date).or_insert((0, 0, 0));
                        entry.0 += reward.amount_micros; // total
                        entry.1 = entry.1.max(reward.amount_micros); // max
                        entry.2 += 1; // count
                    }
                }
            }
            Err(e) => {
                println!("[REWARDS] Error fetching rewards for license {}: {}", license_id, e);
            }
        }
    }

    // Convert to DTOs
    let result: Vec<DailyRewardDto> = daily_data
        .into_iter()
        .map(|(date, (total, max, count))| DailyRewardDto {
            date,
            total_amount_micros: total,
            max_amount_micros: max,
            count,
        })
        .collect();

    println!("[REWARDS] Returning {} daily data points for agent", result.len());
    Ok(result)
}

/// Context for managing rewards state across the application
#[derive(Clone, Copy)]
pub struct RewardsContext {
    pub token: RwSignal<String>,
    pub state: RwSignal<RewardsState>,
    pub license_configs: RwSignal<HashMap<String, LicenseConfig>>,
}

impl RewardsContext {
    pub fn new() -> Self {
        Self {
            token: RwSignal::new(String::new()),
            state: RwSignal::new(RewardsState::default()),
            license_configs: RwSignal::new(HashMap::new()),
        }
    }

    pub fn set_token(&self, token: impl Into<String>) {
        self.token.set(token.into());
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.loading = loading);
    }

    pub fn set_error(&self, error: Option<String>) {
        self.state.update(|s| s.error = error);
    }

    pub fn set_allocations(&self, allocations: Vec<RewardAllocation>, summary: AllocationsSummary) {
        let configs = self.license_configs.get();
        let computed: Vec<ComputedAllocation> = allocations
            .iter()
            .map(|a| ComputedAllocation::compute(a.clone(), configs.get(&a.license_id)))
            .collect();

        self.state.update(|s| {
            s.allocations = allocations;
            s.computed_allocations = computed;
            s.summary = summary;
            s.loading = false;
            s.error = None;
        });
    }

    pub fn set_dashboard_data(&self, balance: i64, api_summary: AllocationsSummaryApi) {
        self.state.update(|s| {
            s.balance = balance;
            s.api_summary = api_summary;
        });
    }

    pub fn set_license_configs(&self, configs: HashMap<String, LicenseConfig>) {
        self.license_configs.set(configs);
        // Recompute allocations with new configs
        let state = self.state.get();
        if !state.allocations.is_empty() {
            let cfgs = self.license_configs.get();
            let computed: Vec<ComputedAllocation> = state
                .allocations
                .iter()
                .map(|a| ComputedAllocation::compute(a.clone(), cfgs.get(&a.license_id)))
                .collect();
            self.state.update(|s| s.computed_allocations = computed);
        }
    }

    pub fn next_page(&self) {
        self.state.update(|s| {
            if s.has_next_page() {
                s.current_page += 1;
            }
        });
    }

    pub fn prev_page(&self) {
        self.state.update(|s| {
            if s.has_prev_page() {
                s.current_page -= 1;
            }
        });
    }

    pub fn go_to_page(&self, page: u32) {
        self.state.update(|s| {
            if page < s.total_pages() {
                s.current_page = page;
            }
        });
    }
}

impl Default for RewardsContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook to access rewards context
pub fn use_rewards() -> RewardsContext {
    expect_context::<RewardsContext>()
}

/// Provider component for RewardsContext
#[component]
pub fn RewardsContextProvider(children: Children) -> impl IntoView {
    let context = RewardsContext::new();
    provide_context(context);
    children()
}
