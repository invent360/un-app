//! Data models and DTOs for uno-api.

mod forecast;
mod import;
mod license;
pub mod marketplace;
mod variant;

pub mod request;
pub mod response;

pub use forecast::*;
pub use import::*;
pub use license::*;
pub use variant::*;

/// Pagination request parameters.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based).
    #[serde(default = "default_page")]
    pub page: u32,
    /// Items per page.
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl PaginationParams {
    /// Calculate SQL OFFSET.
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.per_page
    }

    /// Get the limit (per_page).
    pub fn limit(&self) -> u32 {
        self.per_page
    }
}

/// Paginated response wrapper.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Paginated<T> {
    /// Items in this page.
    pub items: Vec<T>,
    /// Total number of items.
    pub total: i64,
    /// Current page (1-based).
    pub page: u32,
    /// Items per page.
    pub per_page: u32,
    /// Total number of pages.
    pub total_pages: u32,
}

impl<T> Paginated<T> {
    /// Create a new paginated response.
    pub fn new(items: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        let total_pages = ((total as f64) / (params.per_page as f64)).ceil() as u32;
        Self {
            items,
            total,
            page: params.page,
            per_page: params.per_page,
            total_pages,
        }
    }

    /// Check if there's a next page.
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// Check if there's a previous page.
    pub fn has_previous(&self) -> bool {
        self.page > 1
    }
}
