//! Agent repository trait definition

use async_trait::async_trait;
use crate::models::entity::{AgentEntity, NewAgent};

/// Repository trait for Agent operations
#[async_trait]
pub trait AgentRepositoryTrait: Send + Sync {
    /// Save a new agent to the database
    async fn save_agent(&self, new_agent: NewAgent) -> Result<AgentEntity, String>;

    /// Get an agent by its primary key (UUID)
    async fn get_agent_by_id(&self, id: &str) -> Result<Option<AgentEntity>, String>;

    /// Get an agent by email address
    async fn get_agent_by_email(&self, email: &str) -> Result<Option<AgentEntity>, String>;

    /// List all agents
    async fn list_agents(&self) -> Result<Vec<AgentEntity>, String>;

    /// Update an existing agent
    async fn update_agent(&self, agent: AgentEntity) -> Result<AgentEntity, String>;

    /// Delete an agent by ID
    async fn delete_agent(&self, id: &str) -> Result<bool, String>;

    /// Count all agents
    async fn count_agents(&self) -> Result<i64, String>;
}
