use leptos::prelude::*;
use super::types::*;

/// State for the rewards page
#[derive(Debug, Clone)]
pub struct RewardsState {
    pub allocations: Vec<RewardAllocation>,
    pub summary: AllocationsSummary,
    pub current_page: u32,
    pub page_size: u32,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for RewardsState {
    fn default() -> Self {
        Self {
            allocations: vec![],
            summary: AllocationsSummary::default(),
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

/// Stub type for node allocations (original in disabled rewards_logic module)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeAllocations {
    pub node_id: String,
    pub allocations: Vec<RewardAllocation>,
    pub total_amount_micros: i64,
    pub allocation_count: usize,
}

/// Handler for fetching allocations grouped by node
/// NOTE: Temporarily stubbed - rewards logic depends on removed client module
pub async fn fetch_allocations_by_node_handler(
    _token: String,
) -> Result<Vec<NodeAllocations>, String> {
    Ok(vec![])
}

/// Context for managing rewards state across the application
#[derive(Clone, Copy)]
pub struct RewardsContext {
    pub token: RwSignal<String>,
    pub state: RwSignal<RewardsState>,
}

impl RewardsContext {
    pub fn new() -> Self {
        Self {
            token: RwSignal::new(String::new()),
            state: RwSignal::new(RewardsState::default()),
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
        self.state.update(|s| {
            s.allocations = allocations;
            s.summary = summary;
            s.loading = false;
            s.error = None;
        });
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
