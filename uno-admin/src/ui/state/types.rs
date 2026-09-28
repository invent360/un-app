use serde::{Deserialize, Serialize};
use chrono::{Duration, NaiveDate, Utc, Datelike};

/// Period filter for charts and data views
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Period {
    #[default]
    Weekly,
    Monthly,
    Yearly,
}

// ========== Dashboard Overview Types ==========

/// Time granularity for grouping data in charts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TimeGranularity {
    #[default]
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

impl TimeGranularity {
    pub fn label(&self) -> &'static str {
        match self {
            TimeGranularity::Daily => "Daily",
            TimeGranularity::Weekly => "Weekly",
            TimeGranularity::Monthly => "Monthly",
            TimeGranularity::Quarterly => "Quarterly",
            TimeGranularity::Yearly => "Yearly",
        }
    }

    pub fn all() -> &'static [TimeGranularity] {
        &[
            TimeGranularity::Daily,
            TimeGranularity::Weekly,
            TimeGranularity::Monthly,
            TimeGranularity::Quarterly,
            TimeGranularity::Yearly,
        ]
    }
}

/// Date range preset options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DateRangePreset {
    Last7Days,
    #[default]
    Last30Days,
    Last90Days,
    LastYear,
    AllTime,
}

impl DateRangePreset {
    pub fn label(&self) -> &'static str {
        match self {
            DateRangePreset::Last7Days => "Last 7 Days",
            DateRangePreset::Last30Days => "Last 30 Days",
            DateRangePreset::Last90Days => "Last 90 Days",
            DateRangePreset::LastYear => "Last Year",
            DateRangePreset::AllTime => "All Time",
        }
    }

    pub fn all() -> &'static [DateRangePreset] {
        &[
            DateRangePreset::Last7Days,
            DateRangePreset::Last30Days,
            DateRangePreset::Last90Days,
            DateRangePreset::LastYear,
            DateRangePreset::AllTime,
        ]
    }
}

/// Date range with start/end dates
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    pub start: String,  // YYYY-MM-DD
    pub end: String,    // YYYY-MM-DD
    pub preset: DateRangePreset,
}

impl Default for DateRange {
    fn default() -> Self {
        Self::last_30_days()
    }
}

impl DateRange {
    pub fn last_7_days() -> Self {
        let end = Utc::now().date_naive();
        let start = end - Duration::days(7);
        Self {
            start: start.format("%Y-%m-%d").to_string(),
            end: end.format("%Y-%m-%d").to_string(),
            preset: DateRangePreset::Last7Days,
        }
    }

    pub fn last_30_days() -> Self {
        let end = Utc::now().date_naive();
        let start = end - Duration::days(30);
        Self {
            start: start.format("%Y-%m-%d").to_string(),
            end: end.format("%Y-%m-%d").to_string(),
            preset: DateRangePreset::Last30Days,
        }
    }

    pub fn last_90_days() -> Self {
        let end = Utc::now().date_naive();
        let start = end - Duration::days(90);
        Self {
            start: start.format("%Y-%m-%d").to_string(),
            end: end.format("%Y-%m-%d").to_string(),
            preset: DateRangePreset::Last90Days,
        }
    }

    pub fn last_year() -> Self {
        let end = Utc::now().date_naive();
        let start = end - Duration::days(365);
        Self {
            start: start.format("%Y-%m-%d").to_string(),
            end: end.format("%Y-%m-%d").to_string(),
            preset: DateRangePreset::LastYear,
        }
    }

    pub fn all_time() -> Self {
        let end = Utc::now().date_naive();
        // Start from a reasonable date in the past
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        Self {
            start: start.format("%Y-%m-%d").to_string(),
            end: end.format("%Y-%m-%d").to_string(),
            preset: DateRangePreset::AllTime,
        }
    }

    pub fn from_preset(preset: DateRangePreset) -> Self {
        match preset {
            DateRangePreset::Last7Days => Self::last_7_days(),
            DateRangePreset::Last30Days => Self::last_30_days(),
            DateRangePreset::Last90Days => Self::last_90_days(),
            DateRangePreset::LastYear => Self::last_year(),
            DateRangePreset::AllTime => Self::all_time(),
        }
    }
}

/// Top performer entry with ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopPerformer {
    pub id: String,
    pub name: String,
    pub earnings: f64,
    pub rank: u8,  // 1=Gold, 2=Silver, 3=Bronze
}

/// Grouped earnings for a time period (for bar chart)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedEarnings {
    pub period: String,  // "2024-01-15" or "2024-W03" or "2024-01" etc.
    pub amount: f64,
}

/// Earnings distribution for pie chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsDistribution {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub percentage: f64,
}

/// Licenses overview data for dashboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LicensesOverviewData {
    pub total_licenses: usize,
    pub online_count: usize,
    pub average_uptime: f64,
    pub total_earnings: f64,
    pub available_earnings: f64,  // Unclaimed balance
    pub top_performers: Vec<TopPerformer>,
    pub earnings_distribution: Vec<EarningsDistribution>,  // For pie chart
    pub earnings_over_time: Vec<GroupedEarnings>,  // For bar chart
}

/// Agents overview data for dashboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentsOverviewData {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_commissions: f64,
    pub available_commissions: f64,
    pub top_performers: Vec<TopPerformer>,
    pub commissions_distribution: Vec<EarningsDistribution>,
    pub commissions_over_time: Vec<GroupedEarnings>,
}

impl Period {
    pub fn label(&self) -> &'static str {
        match self {
            Period::Weekly => "Weekly",
            Period::Monthly => "Monthly",
            Period::Yearly => "Yearly",
        }
    }

    pub fn all() -> &'static [Period] {
        &[Period::Weekly, Period::Monthly, Period::Yearly]
    }

    pub fn to_string(&self) -> String {
        match self {
            Period::Weekly => "weekly".to_string(),
            Period::Monthly => "monthly".to_string(),
            Period::Yearly => "yearly".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "weekly" => Some(Period::Weekly),
            "monthly" => Some(Period::Monthly),
            "yearly" => Some(Period::Yearly),
            _ => None,
        }
    }
}

/// Stat card variant for different icons/colors
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatVariant {
    Menus,
    Orders,
    Clients,
    Revenue,
}

impl StatVariant {
    pub fn css_class(&self) -> &'static str {
        match self {
            StatVariant::Menus => "stat-card--menus",
            StatVariant::Orders => "stat-card--orders",
            StatVariant::Clients => "stat-card--clients",
            StatVariant::Revenue => "stat-card--revenue",
        }
    }
}

/// Dashboard statistics for the home page
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_menus: u32,
    pub total_orders: u32,
    pub total_clients: u32,
    pub total_revenue: f64,
}

impl Default for DashboardStats {
    fn default() -> Self {
        Self {
            total_menus: 140,
            total_orders: 175,
            total_clients: 263,
            total_revenue: 13755.0,
        }
    }
}

/// Single data point for charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: f64,
}

impl ChartDataPoint {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Revenue data for the revenue chart section
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RevenueData {
    pub total: f64,
    pub average: f64,
    pub data_points: Vec<ChartDataPoint>,
}

impl Default for RevenueData {
    fn default() -> Self {
        Self {
            total: 7258.0,
            average: 1139.0,
            data_points: vec![
                ChartDataPoint::new("S", 8000.0),
                ChartDataPoint::new("M", 12000.0),
                ChartDataPoint::new("T", 10000.0),
                ChartDataPoint::new("W", 15000.0),
                ChartDataPoint::new("T", 14000.0),
                ChartDataPoint::new("F", 16000.0),
                ChartDataPoint::new("S", 13000.0),
            ],
        }
    }
}

/// Orders data for the orders chart section
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrdersChartData {
    pub total: f64,
    pub data_points: Vec<ChartDataPoint>,
}

impl Default for OrdersChartData {
    fn default() -> Self {
        Self {
            total: 11833.90,
            data_points: vec![
                ChartDataPoint::new("Jan", 5000.0),
                ChartDataPoint::new("Feb", 8000.0),
                ChartDataPoint::new("Mar", 15000.0),
                ChartDataPoint::new("Apr", 12000.0),
                ChartDataPoint::new("May", 18000.0),
                ChartDataPoint::new("Jun", 14000.0),
                ChartDataPoint::new("Jul", 10000.0),
            ],
        }
    }
}

/// Single order item in the orders list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: String,
    pub user_name: String,
    pub user_avatar: Option<String>,
    pub date: String,
    pub amount: f64,
    pub is_new: bool,
}

impl OrderItem {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Self {
                id: "1".to_string(),
                user_name: "John Doe".to_string(),
                user_avatar: None,
                date: "Mar 24th, 2024".to_string(),
                amount: 6.50,
                is_new: true,
            },
            Self {
                id: "2".to_string(),
                user_name: "Tatiana Rose".to_string(),
                user_avatar: None,
                date: "Mar 23th, 2024".to_string(),
                amount: 6.50,
                is_new: true,
            },
            Self {
                id: "3".to_string(),
                user_name: "Alex Smith".to_string(),
                user_avatar: None,
                date: "Mar 22th, 2024".to_string(),
                amount: 6.50,
                is_new: true,
            },
        ]
    }
}

/// Analytics sales data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalesData {
    pub total_sales: u32,
    pub average_sales: u32,
    pub data_points: Vec<ChartDataPoint>,
}

impl Default for SalesData {
    fn default() -> Self {
        Self {
            total_sales: 18000,
            average_sales: 1139,
            data_points: vec![
                ChartDataPoint::new("S", 150.0),
                ChartDataPoint::new("M", 200.0),
                ChartDataPoint::new("T", 180.0),
                ChartDataPoint::new("W", 250.0),
                ChartDataPoint::new("T", 220.0),
                ChartDataPoint::new("F", 280.0),
                ChartDataPoint::new("S", 240.0),
            ],
        }
    }
}

/// Product statistics for analytics page
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductStat {
    pub id: String,
    pub name: String,
    pub category: String,
    pub image_url: Option<String>,
    pub sales_count: u32,
    pub change_percent: f64,
}

impl ProductStat {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Self {
                id: "1".to_string(),
                name: "Shopsticka Soya".to_string(),
                category: "Sushi".to_string(),
                image_url: None,
                sales_count: 654,
                change_percent: 14.0,
            },
            Self {
                id: "2".to_string(),
                name: "Cheese Spinach".to_string(),
                category: "Pizza".to_string(),
                image_url: None,
                sales_count: 258,
                change_percent: 4.0,
            },
            Self {
                id: "3".to_string(),
                name: "Italian Chowmin".to_string(),
                category: "Pasta".to_string(),
                image_url: None,
                sales_count: 82,
                change_percent: -2.0,
            },
        ]
    }
}

/// User profile information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub avatar_url: Option<String>,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            name: "Alex".to_string(),
            avatar_url: None,
        }
    }
}

/// CMS sub-navigation items
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CmsTab {
    Content,
    Schemas,
    Reviews,
    Publish,
}

impl CmsTab {
    pub fn label(&self) -> &'static str {
        match self {
            CmsTab::Content => "Content",
            CmsTab::Schemas => "Schemas",
            CmsTab::Reviews => "Reviews",
            CmsTab::Publish => "Publish",
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            CmsTab::Content => "/content",
            CmsTab::Schemas => "/schemas",
            CmsTab::Reviews => "/reviews",
            CmsTab::Publish => "/publish",
        }
    }

    pub fn all() -> &'static [CmsTab] {
        &[
            CmsTab::Content,
            CmsTab::Schemas,
            CmsTab::Reviews,
            CmsTab::Publish,
        ]
    }
}

/// Navigation tab identifiers
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NavTab {
    #[default]
    Home,
    Marketplace,
    Agents,
    Licenses,
    Nodes,
    Rewards,
    Jobs,
    Cms,
    Settings,
}

impl NavTab {
    pub fn label(&self) -> &'static str {
        match self {
            NavTab::Home => "Home",
            NavTab::Marketplace => "Marketplace",
            NavTab::Agents => "Agents",
            NavTab::Licenses => "Licenses",
            NavTab::Nodes => "Nodes",
            NavTab::Rewards => "Rewards",
            NavTab::Jobs => "Jobs",
            NavTab::Cms => "CMS",
            NavTab::Settings => "Settings",
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            NavTab::Home => "/",
            NavTab::Marketplace => "/marketplace",
            NavTab::Agents => "/agents",
            NavTab::Licenses => "/licenses",
            NavTab::Nodes => "/nodes",
            NavTab::Rewards => "/rewards",
            NavTab::Jobs => "/jobs",
            NavTab::Cms => "/cms",
            NavTab::Settings => "/settings",
        }
    }

    /// Returns true if this tab has a submenu
    pub fn has_submenu(&self) -> bool {
        matches!(self, NavTab::Cms)
    }

    pub fn all() -> &'static [NavTab] {
        &[
            NavTab::Home,
            NavTab::Marketplace,
            NavTab::Nodes,
            NavTab::Licenses,
            NavTab::Rewards,
            NavTab::Jobs,
            NavTab::Agents,
            NavTab::Cms,
            NavTab::Settings,
        ]
    }
}
