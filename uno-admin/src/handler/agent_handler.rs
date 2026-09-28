//! Handlers for agent CRUD operations

use leptos::prelude::*;
use server_fn::codec::PostUrl;
use serde::{Deserialize, Serialize};
use crate::models::entity::{AgentEntity, NewAgent};

/// Data for creating a new agent via form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentForm {
    pub name: String,
    pub email: String,
    pub country: String,
    pub commission_percent: f64,
    pub referral_code: Option<String>,
}

impl From<CreateAgentForm> for NewAgent {
    fn from(form: CreateAgentForm) -> Self {
        NewAgent {
            name: form.name,
            email: form.email,
            country: form.country,
            commission_percent: form.commission_percent,
            referral_code: form.referral_code,
        }
    }
}

/// Server function to create a new agent
#[server(CreateAgent, "/api")]
pub async fn create_agent(form: CreateAgentForm) -> Result<AgentEntity, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = AgentService::new(pool);
    service
        .create_agent(form.into())
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to list all agents
#[server(ListAgents, "/api")]
pub async fn list_agents() -> Result<Vec<AgentEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = AgentService::new(pool);
    service
        .list_agents()
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to get a single agent by ID
#[server(GetAgent, "/api")]
pub async fn get_agent(id: String) -> Result<Option<AgentEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = AgentService::new(pool);
    service
        .get_agent(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Agent with license count for list display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentWithStats {
    pub id: String,
    pub name: String,
    pub email: String,
    pub country: String,
    pub commission_percent: f64,
    pub license_count: usize,
    pub online_count: usize,
    pub created_at: String,
}

/// Server function to get agent with licenses
#[server(GetAgentWithLicenses, "/api")]
pub async fn get_agent_with_licenses(
    id: String,
) -> Result<Option<(AgentEntity, Vec<crate::models::entity::LicenseEntity>)>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = AgentService::new(pool);
    service
        .get_agent_with_licenses(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to delete an agent
#[server(DeleteAgent, "/api")]
pub async fn delete_agent(id: String) -> Result<bool, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = AgentService::new(pool);
    service
        .delete_agent(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to import licenses from CSV
#[server(ImportLicensesCsv, "/api", endpoint = "import_licenses_csv", input = PostUrl)]
pub async fn import_licenses_csv(agent_id: String, csv_content: String) -> Result<usize, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::AgentService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = AgentService::new(pool);
    service
        .import_licenses_csv(&agent_id, &csv_content)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// State for the agents page
#[derive(Debug, Clone, Default)]
pub struct AgentsPageState {
    pub agents: Vec<AgentEntity>,
    pub loading: bool,
    pub error: Option<String>,
    pub show_create_form: bool,
}

/// Context for managing agents state
#[derive(Clone, Copy)]
pub struct AgentsContext {
    pub state: RwSignal<AgentsPageState>,
}

impl AgentsContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(AgentsPageState::default()),
        }
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.loading = loading);
    }

    pub fn set_error(&self, error: Option<String>) {
        self.state.update(|s| s.error = error);
    }

    pub fn set_agents(&self, agents: Vec<AgentEntity>) {
        self.state.update(|s| {
            s.agents = agents;
            s.loading = false;
            s.error = None;
        });
    }

    pub fn toggle_create_form(&self) {
        self.state.update(|s| s.show_create_form = !s.show_create_form);
    }

    pub fn hide_create_form(&self) {
        self.state.update(|s| s.show_create_form = false);
    }

    pub fn add_agent(&self, agent: AgentEntity) {
        self.state.update(|s| {
            s.agents.insert(0, agent);
            s.show_create_form = false;
        });
    }

    pub fn remove_agent(&self, id: &str) {
        self.state.update(|s| {
            s.agents.retain(|a| a.id != id);
        });
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
