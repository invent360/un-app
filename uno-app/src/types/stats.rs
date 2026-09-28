//! Statistics-related type definitions

use serde::{Deserialize, Serialize};

/// Network overview statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_licenses: i64,
    pub claimed_licenses: i64,
    pub available_licenses: i64,
    pub total_variants: i64,
    pub average_user_share: f64,
    pub utilization_percentage: f64,
}

impl NetworkStats {
    /// Create NetworkStats from basic counts
    pub fn new(
        total_licenses: i64,
        claimed_licenses: i64,
        total_variants: i64,
        average_user_share: f64,
    ) -> Self {
        let available_licenses = total_licenses - claimed_licenses;
        let utilization_percentage = if total_licenses > 0 {
            (claimed_licenses as f64 / total_licenses as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_licenses,
            claimed_licenses,
            available_licenses,
            total_variants,
            average_user_share,
            utilization_percentage,
        }
    }
}

/// Visitor statistics for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorStats {
    pub period: String,
    pub total_visitors: i64,
    pub unique_visitors: i64,
    pub countries: Vec<CountryStats>,
}

/// Country-level visitor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryStats {
    pub country_code: String,
    pub visitor_count: i64,
    pub flag: String,
}

/// Database row for country stats (without computed fields)
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CountryStatsRow {
    pub country_code: String,
    pub visitor_count: i64,
}

#[cfg(feature = "ssr")]
impl CountryStatsRow {
    /// Convert to CountryStats with computed flag
    pub fn into_stats(self) -> CountryStats {
        let flag = country_code_to_flag(&self.country_code);
        CountryStats {
            country_code: self.country_code,
            visitor_count: self.visitor_count,
            flag,
        }
    }
}

/// Convert ISO 3166-1 alpha-2 country code to flag emoji
pub fn country_code_to_flag(code: &str) -> String {
    if code == "Unknown" || code.len() != 2 {
        return "🏳️".to_string(); // White flag for unknown
    }

    // Convert each letter to regional indicator symbol
    // A-Z maps to U+1F1E6 - U+1F1FF
    code.to_uppercase()
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                let offset = c as u32 - 'A' as u32;
                char::from_u32(0x1F1E6 + offset)
            } else {
                None
            }
        })
        .collect()
}

impl CountryStats {
    /// Get country name from code (basic mapping)
    pub fn country_name(&self) -> &str {
        match self.country_code.as_str() {
            "PH" => "Philippines",
            "PK" => "Pakistan",
            "IN" => "India",
            "NG" => "Nigeria",
            "KE" => "Kenya",
            "GH" => "Ghana",
            "US" => "United States",
            "GB" => "United Kingdom",
            "ID" => "Indonesia",
            "VN" => "Vietnam",
            "BD" => "Bangladesh",
            "EG" => "Egypt",
            "ZA" => "South Africa",
            "Unknown" => "Unknown",
            _ => &self.country_code,
        }
    }
}

/// Earnings statistics by tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierStats {
    pub tier_name: String,
    pub user_share: i32,
    pub operator_share: i32,
    pub license_count: i64,
    pub claimed_count: i64,
    pub min_earnings: Option<f64>,
    pub max_earnings: Option<f64>,
}

/// Period filter for statistics
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum PeriodFilter {
    Daily,
    #[default]
    Weekly,
    Monthly,
    AllTime,
}

impl PeriodFilter {
    /// Get the number of days for this period
    pub fn days(&self) -> i32 {
        match self {
            PeriodFilter::Daily => 1,
            PeriodFilter::Weekly => 7,
            PeriodFilter::Monthly => 30,
            PeriodFilter::AllTime => 365 * 10, // 10 years
        }
    }

    /// Get display name for the period
    pub fn display_name(&self) -> &str {
        match self {
            PeriodFilter::Daily => "Today",
            PeriodFilter::Weekly => "This Week",
            PeriodFilter::Monthly => "This Month",
            PeriodFilter::AllTime => "All Time",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "daily" | "day" | "today" => PeriodFilter::Daily,
            "weekly" | "week" => PeriodFilter::Weekly,
            "monthly" | "month" => PeriodFilter::Monthly,
            "all" | "alltime" | "all_time" => PeriodFilter::AllTime,
            _ => PeriodFilter::default(),
        }
    }
}

/// Complete dashboard statistics (visitor-focused)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub visitors: VisitorStats,
    pub top_countries: Vec<CountryStats>,
}

/// Stats query parameters with pagination support
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsQueryParams {
    /// Period filter (daily, weekly, monthly, all)
    #[serde(default)]
    pub period: Option<String>,
    /// Country limit for top countries
    #[serde(default)]
    pub country_limit: Option<i32>,
    /// Page number for pagination (1-indexed)
    #[serde(default)]
    pub page: Option<i32>,
    /// Items per page
    #[serde(default)]
    pub per_page: Option<i32>,
}

impl StatsQueryParams {
    /// Get period filter from params
    pub fn period_filter(&self) -> PeriodFilter {
        self.period
            .as_ref()
            .map(|p| PeriodFilter::from_str(p))
            .unwrap_or_default()
    }

    /// Get country limit with default
    pub fn get_country_limit(&self) -> i32 {
        self.country_limit.unwrap_or(10)
    }

    /// Get page number with default
    pub fn get_page(&self) -> i32 {
        self.page.unwrap_or(1).max(1)
    }

    /// Get items per page with default and max limit
    pub fn get_per_page(&self) -> i32 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }

    /// Calculate SQL offset
    pub fn offset(&self) -> i64 {
        ((self.get_page() - 1) * self.get_per_page()) as i64
    }

    /// Calculate SQL limit
    pub fn limit(&self) -> i64 {
        self.get_per_page() as i64
    }
}
