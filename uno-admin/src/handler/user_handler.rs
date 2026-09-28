//! Handlers for agent and ULO operations with reactive state

use leptos::prelude::*;
use crate::api::types::RewardAllocation;
use crate::models::{AgentPerformance, LicenseConfig};
use crate::logic::{calculate_agent_performance, calculate_ulo_performance, UloPerformance};
use std::collections::HashMap;

/// State for the agents page
#[derive(Debug, Clone, Default)]
pub struct AgentsState {
    pub agents: Vec<AgentPerformance>,
    pub current_page: u32,
    pub page_size: u32,
    pub loading: bool,
    pub error: Option<String>,
    pub sort_by: AgentSortBy,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AgentSortBy {
    #[default]
    Commission,
    Earnings,
    Licenses,
    Uptime,
}

impl AgentsState {
    /// Get sorted agents
    pub fn sorted_agents(&self) -> Vec<&AgentPerformance> {
        let mut agents: Vec<&AgentPerformance> = self.agents.iter().collect();
        match self.sort_by {
            AgentSortBy::Commission => {
                agents.sort_by(|a, b| b.agent_share_micros.cmp(&a.agent_share_micros));
            }
            AgentSortBy::Earnings => {
                agents.sort_by(|a, b| b.total_earnings_micros.cmp(&a.total_earnings_micros));
            }
            AgentSortBy::Licenses => {
                agents.sort_by(|a, b| b.license_count.cmp(&a.license_count));
            }
            AgentSortBy::Uptime => {
                agents.sort_by(|a, b| b.avg_uptime.partial_cmp(&a.avg_uptime).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
        agents
    }

    /// Get agents for the current page
    pub fn current_page_agents(&self) -> Vec<&AgentPerformance> {
        let sorted = self.sorted_agents();
        let start = (self.current_page * self.page_size) as usize;
        let end = std::cmp::min(start + self.page_size as usize, sorted.len());
        if start >= sorted.len() {
            vec![]
        } else {
            sorted[start..end].to_vec()
        }
    }

    pub fn total_pages(&self) -> u32 {
        if self.agents.is_empty() {
            0
        } else {
            ((self.agents.len() as u32 - 1) / self.page_size) + 1
        }
    }

    pub fn has_next_page(&self) -> bool {
        self.current_page + 1 < self.total_pages()
    }

    pub fn has_prev_page(&self) -> bool {
        self.current_page > 0
    }

    /// Get total commission for all agents
    pub fn total_commission(&self) -> i64 {
        self.agents.iter().map(|a| a.agent_share_micros).sum()
    }

    /// Get total earnings across all agents
    pub fn total_earnings(&self) -> i64 {
        self.agents.iter().map(|a| a.total_earnings_micros).sum()
    }
}

/// Context for managing agents state across the application
#[derive(Clone, Copy)]
pub struct AgentsContext {
    pub state: RwSignal<AgentsState>,
    pub allocations: RwSignal<Vec<RewardAllocation>>,
    pub license_configs: RwSignal<HashMap<String, LicenseConfig>>,
}

impl AgentsContext {
    pub fn new() -> Self {
        let mut initial_state = AgentsState::default();
        initial_state.page_size = 10;

        Self {
            state: RwSignal::new(initial_state),
            allocations: RwSignal::new(vec![]),
            license_configs: RwSignal::new(HashMap::new()),
        }
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.loading = loading);
    }

    pub fn set_error(&self, error: Option<String>) {
        self.state.update(|s| s.error = error);
    }

    /// Set allocations and license configs, then compute agent performance
    pub fn set_data(&self, allocations: Vec<RewardAllocation>, configs: HashMap<String, LicenseConfig>) {
        let agents = calculate_agent_performance(&allocations, &configs);
        self.allocations.set(allocations);
        self.license_configs.set(configs);
        self.state.update(|s| {
            s.agents = agents;
            s.loading = false;
            s.error = None;
        });
    }

    /// Update allocations only (recomputes agent performance)
    pub fn set_allocations(&self, allocations: Vec<RewardAllocation>) {
        let configs = self.license_configs.get();
        let agents = calculate_agent_performance(&allocations, &configs);
        self.allocations.set(allocations);
        self.state.update(|s| {
            s.agents = agents;
            s.loading = false;
            s.error = None;
        });
    }

    /// Update license configs only (recomputes agent performance)
    pub fn set_license_configs(&self, configs: HashMap<String, LicenseConfig>) {
        let allocs = self.allocations.get();
        let agents = calculate_agent_performance(&allocs, &configs);
        self.license_configs.set(configs);
        self.state.update(|s| {
            s.agents = agents;
        });
    }

    pub fn set_sort_by(&self, sort_by: AgentSortBy) {
        self.state.update(|s| {
            s.sort_by = sort_by;
            s.current_page = 0;
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

    /// Get a specific agent by name
    pub fn get_agent(&self, agent_name: &str) -> Option<AgentPerformance> {
        self.state.get().agents
            .iter()
            .find(|a| a.agent_name == agent_name)
            .cloned()
    }

    /// Get ULOs for a specific agent
    pub fn get_agent_ulos(&self, agent_name: &str) -> Vec<String> {
        crate::logic::get_agent_ulos(agent_name, &self.license_configs.get())
    }

    /// Get ULO performance data
    pub fn get_ulo_performance(&self) -> Vec<UloPerformance> {
        calculate_ulo_performance(&self.allocations.get(), &self.license_configs.get())
    }

    /// Get top agents by commission
    pub fn get_top_agents(&self, limit: usize) -> Vec<AgentPerformance> {
        crate::logic::get_top_agents(self.state.get().agents.clone(), limit)
    }
}

impl Default for AgentsContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook to access agents context
pub fn use_agents() -> AgentsContext {
    expect_context::<AgentsContext>()
}

/// Provider component for AgentsContext
#[component]
pub fn AgentsContextProvider(children: Children) -> impl IntoView {
    let context = AgentsContext::new();
    provide_context(context);
    children()
}
