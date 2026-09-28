use leptos::prelude::*;
use super::types::*;

/// Global dashboard state using Leptos RwSignals
#[derive(Clone, Copy)]
pub struct DashboardState {
    // Dashboard data
    pub stats: RwSignal<DashboardStats>,
    pub revenue_data: RwSignal<RevenueData>,
    pub orders_chart_data: RwSignal<OrdersChartData>,
    pub orders_list: RwSignal<Vec<OrderItem>>,
    pub sales_data: RwSignal<SalesData>,
    pub products_list: RwSignal<Vec<ProductStat>>,

    // UI state
    pub revenue_period: RwSignal<Period>,
    pub orders_period: RwSignal<Period>,
    pub analytics_period: RwSignal<Period>,
    pub active_tab: RwSignal<NavTab>,

    // User
    pub user: RwSignal<UserProfile>,

    // Loading states
    pub is_loading: RwSignal<bool>,

    // Notification badges
    pub jobs_new_count: RwSignal<u32>,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            stats: RwSignal::new(DashboardStats::default()),
            revenue_data: RwSignal::new(RevenueData::default()),
            orders_chart_data: RwSignal::new(OrdersChartData::default()),
            orders_list: RwSignal::new(OrderItem::mock_data()),
            sales_data: RwSignal::new(SalesData::default()),
            products_list: RwSignal::new(ProductStat::mock_data()),
            revenue_period: RwSignal::new(Period::Weekly),
            orders_period: RwSignal::new(Period::Monthly),
            analytics_period: RwSignal::new(Period::Weekly),
            active_tab: RwSignal::new(NavTab::Home),
            user: RwSignal::new(UserProfile::default()),
            is_loading: RwSignal::new(false),
            jobs_new_count: RwSignal::new(0),
        }
    }

    /// Set the jobs notification badge count
    pub fn set_jobs_badge(&self, count: u32) {
        self.jobs_new_count.set(count);
    }

    /// Clear the jobs notification badge
    pub fn clear_jobs_badge(&self) {
        self.jobs_new_count.set(0);
    }

    /// Add to the jobs notification badge count
    pub fn add_jobs_badge(&self, count: u32) {
        self.jobs_new_count.update(|c| *c += count);
    }

    pub fn set_active_tab(&self, tab: NavTab) {
        self.active_tab.set(tab);
    }

    pub fn set_revenue_period(&self, period: Period) {
        self.revenue_period.set(period);
    }

    pub fn set_orders_period(&self, period: Period) {
        self.orders_period.set(period);
    }

    pub fn set_analytics_period(&self, period: Period) {
        self.analytics_period.set(period);
    }
}

impl Default for DashboardState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide dashboard state to the component tree
pub fn provide_dashboard_state() {
    provide_context(DashboardState::new());
}

/// Get dashboard state from context
pub fn use_dashboard() -> DashboardState {
    expect_context::<DashboardState>()
}
