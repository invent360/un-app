//! Home page with comprehensive dashboard overview
//! Uses SSR Resources for data loading

use leptos::prelude::*;
use crate::components::layout::Header;
use crate::components::common::progress_spinner::{ProgressSpinner, SpinnerSize};
use crate::components::overview::{LicensesOverviewSection, AgentsOverviewSection};
use crate::state::{
    DateRange, TimeGranularity, LicensesOverviewData, AgentsOverviewData
};
use crate::models::LicenseConfig;
use crate::logic::calculate_licenses_overview;
use crate::handler::{
    get_agents_overview_from_db, AgentWithLicenses, calculate_agents_overview_from_db,
    get_rewards_from_db,
    get_uno_licenses_summary, get_allocations_summary,
    get_daily_incentive_totals, run_sync_job,
};
use crate::storage::load_license_configs;
use crate::api::types::RewardAllocation;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Raw data fetched from server (for recalculation on filter change)
#[derive(Clone, Debug, Serialize, Deserialize)]
struct RawDashboardData {
    db_agents: Vec<AgentWithLicenses>,
    db_rewards: Vec<RewardAllocation>,
    #[serde(skip)]
    local_configs: HashMap<String, LicenseConfig>,
}

/// Calculate overview data from raw data
fn calculate_overview_data(
    raw: &RawDashboardData,
    date_range: &DateRange,
    granularity: TimeGranularity,
) -> (LicensesOverviewData, AgentsOverviewData) {
    let licenses_overview = calculate_licenses_overview(
        &[],  // No API licenses - using DB data
        &raw.db_rewards,
        &raw.local_configs,
        0,    // No balance from API
        date_range,
        granularity,
    );

    let agents_overview = calculate_agents_overview_from_db(
        &raw.db_agents,
        &raw.db_rewards,
        date_range,
        granularity,
    );

    (licenses_overview, agents_overview)
}

/// Home page with comprehensive overview
#[component]
pub fn HomePage() -> impl IntoView {
    // Filter signals - shared between license and agent sections
    let date_range = RwSignal::new(DateRange::default());
    let granularity = RwSignal::new(TimeGranularity::Daily);

    // ========================================
    // Daily Incentives Chart (Last 30 Days)
    // ========================================

    // Signal to track if sync is in progress
    let syncing_incentives = RwSignal::new(false);

    // Resource for fetching daily incentive totals (loads immediately)
    let incentive_data = Resource::new(
        || (),
        |_| async move {
            get_daily_incentive_totals().await
                .map_err(|e| e.to_string())
        }
    );

    // Refetch trigger signal
    let refetch_trigger = RwSignal::new(0u32);

    // Pass DailyIncentive data directly to the chart (includes license_count for tooltips)
    let chart_data = Signal::derive(move || {
        // Touch refetch_trigger to make this reactive
        let _ = refetch_trigger.get();

        incentive_data.get().map(|result| {
            result.map_err(|e| e.to_string())
        })
    });

    // Signal for syncing status
    let syncing_signal = Signal::derive(move || syncing_incentives.get());

    // Effect to sync recent days' incentives on page load
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::spawn_local;
        use chrono::Duration;

        Effect::new(move |prev: Option<()>| {
            // Only run once on mount
            if prev.is_some() {
                return;
            }

            syncing_incentives.set(true);

            spawn_local(async move {
                // Sync the last 7 days to catch any missed data
                let now = chrono::Utc::now();
                let mut success_count = 0;

                for days_ago in 0..7 {
                    let date = (now - Duration::days(days_ago)).format("%Y-%m-%d").to_string();

                    match run_sync_job(date).await {
                        Ok(_job) => {
                            success_count += 1;
                        }
                        Err(e) => {
                            // Log but continue with other dates
                            web_sys::console::log_1(&format!("Sync error for {}: {}", days_ago, e).into());
                        }
                    }

                    // Small delay between syncs to avoid overwhelming the API
                    gloo_timers::future::TimeoutFuture::new(500).await;
                }

                // Wait for jobs to complete, then refetch
                if success_count > 0 {
                    gloo_timers::future::TimeoutFuture::new(3000).await;

                    // Refetch the incentive data
                    if let Ok(_) = get_daily_incentive_totals().await {
                        refetch_trigger.update(|v| *v += 1);
                    }
                }

                syncing_incentives.set(false);
            });
        });
    }

    // Fetch UNO licenses summary from API
    let licenses_summary_resource = Resource::new(
        || (),
        |_| async move {
            get_uno_licenses_summary().await
                .map_err(|e| e.to_string())
        }
    );

    // Fetch allocations summary from API
    let allocations_summary_resource = Resource::new(
        || (),
        |_| async move {
            get_allocations_summary().await
                .map_err(|e| e.to_string())
        }
    );

    // Convert Resources to Signals for passing to components
    let licenses_summary_signal = Signal::derive(move || licenses_summary_resource.get());
    let allocations_summary_signal = Signal::derive(move || allocations_summary_resource.get());

    // Use Resource for SSR-compatible data loading
    // This will fetch data on the server and serialize it to the client
    let dashboard_data = Resource::new(
        || (),  // No source - fetch once
        |_| async move {
            // Fetch data from database
            let db_agents = get_agents_overview_from_db().await.unwrap_or_default();
            let db_rewards = get_rewards_from_db().await.unwrap_or_default();

            // Load local configs (client-side only, empty on server)
            #[cfg(target_arch = "wasm32")]
            let local_configs: HashMap<String, LicenseConfig> = load_license_configs()
                .into_iter()
                .map(|c| (c.license_id.clone(), c))
                .collect();

            #[cfg(not(target_arch = "wasm32"))]
            let local_configs: HashMap<String, LicenseConfig> = HashMap::new();

            RawDashboardData {
                db_agents,
                db_rewards,
                local_configs,
            }
        },
    );

    // Derived signals for the overview sections
    let licenses_data = Signal::derive(move || {
        let current_date_range = date_range.get();
        let current_granularity = granularity.get();

        dashboard_data.get().map(|raw| {
            let (licenses, _) = calculate_overview_data(&raw, &current_date_range, current_granularity);
            licenses
        })
    });

    let agents_data = Signal::derive(move || {
        let current_date_range = date_range.get();
        let current_granularity = granularity.get();

        dashboard_data.get().map(|raw| {
            let (_, agents) = calculate_overview_data(&raw, &current_date_range, current_granularity);
            agents
        })
    });

    // Loading is true while resource is pending
    let loading = Signal::derive(move || dashboard_data.get().is_none());

    view! {
        <div>
            <Header title="Dashboard".to_string() show_search=false />

            <div class="px-4 py-4 space-y-6">
                // Loading state for overview sections
                <Suspense fallback=move || view! {
                    <div class="flex items-center justify-center min-h-[60vh]">
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }.into_any()>
                    {move || {
                        // Show content once data is loaded
                        view! {
                            <>
                                <LicensesOverviewSection
                                    data=licenses_data
                                    date_range=date_range
                                    granularity=granularity
                                    loading=loading
                                    licenses_summary=licenses_summary_signal
                                    allocations_summary=allocations_summary_signal
                                    daily_incentives=chart_data
                                    syncing_incentives=syncing_signal
                                />
                                <AgentsOverviewSection
                                    data=agents_data
                                    date_range=date_range
                                    granularity=granularity
                                    loading=loading
                                />
                            </>
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}
