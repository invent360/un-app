//! Business logic for agent operations

use crate::db::DbPool;
use crate::models::entity::{AgentEntity, LicenseEntity, NewAgent, NewLicense};
use crate::repository::traits::{AgentRepositoryTrait, LicenseRepositoryTrait};

use crate::repository::postgres::{PgAgentRepository as AgentRepository, PgLicenseRepository as LicenseRepository};

/// Agent service for business operations
pub struct AgentService {
    agent_repo: AgentRepository,
    license_repo: LicenseRepository,
}

impl AgentService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            agent_repo: AgentRepository::new(pool.clone()),
            license_repo: LicenseRepository::new(pool),
        }
    }

    /// Create a new agent
    pub async fn create_agent(&self, new_agent: NewAgent) -> Result<AgentEntity, String> {
        // Check if email already exists
        if let Some(_) = self
            .agent_repo
            .get_agent_by_email(&new_agent.email)
            .await
            .map_err(|e| e.to_string())?
        {
            return Err(format!("Agent with email '{}' already exists", new_agent.email));
        }

        self.agent_repo
            .save_agent(new_agent)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get all agents
    pub async fn list_agents(&self) -> Result<Vec<AgentEntity>, String> {
        self.agent_repo
            .list_agents()
            .await
            .map_err(|e| e.to_string())
    }

    /// Get an agent by ID
    pub async fn get_agent(&self, id: &str) -> Result<Option<AgentEntity>, String> {
        self.agent_repo
            .get_agent_by_id(id)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get an agent with their recruited licenses
    pub async fn get_agent_with_licenses(
        &self,
        id: &str,
    ) -> Result<Option<(AgentEntity, Vec<LicenseEntity>)>, String> {
        let agent = self
            .agent_repo
            .get_agent_by_id(id)
            .await
            .map_err(|e| e.to_string())?;

        match agent {
            Some(agent) => {
                let licenses = self
                    .license_repo
                    .get_licenses_by_agent_id(&agent.id)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Some((agent, licenses)))
            }
            None => Ok(None),
        }
    }

    /// Update an agent
    pub async fn update_agent(&self, agent: AgentEntity) -> Result<AgentEntity, String> {
        self.agent_repo
            .update_agent(agent)
            .await
            .map_err(|e| e.to_string())
    }

    /// Delete an agent (will cascade delete their licenses)
    pub async fn delete_agent(&self, id: &str) -> Result<bool, String> {
        self.agent_repo
            .delete_agent(id)
            .await
            .map_err(|e| e.to_string())
    }

    /// Count agents
    pub async fn count_agents(&self) -> Result<i64, String> {
        self.agent_repo
            .count_agents()
            .await
            .map_err(|e| e.to_string())
    }

    /// Import licenses from CSV for a specific agent
    pub async fn import_licenses_csv(
        &self,
        agent_id: &str,
        csv_content: &str,
    ) -> Result<usize, String> {
        // Verify agent exists
        let agent = self
            .agent_repo
            .get_agent_by_id(agent_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Agent not found: {}", agent_id))?;

        // Parse CSV
        let licenses = parse_license_csv_for_agent(csv_content, &agent.id)?;

        // Bulk save
        self.license_repo
            .bulk_save_licenses(licenses)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get agent performance summary
    pub async fn get_agent_performance(&self, agent_id: &str) -> Result<AgentPerformance, String> {
        let (agent, licenses) = self
            .get_agent_with_licenses(agent_id)
            .await?
            .ok_or_else(|| format!("Agent not found: {}", agent_id))?;

        let total_licenses = licenses.len();
        let online_licenses = licenses.iter().filter(|l| l.is_online).count();
        let avg_uptime = if total_licenses > 0 {
            licenses.iter().map(|l| l.uptime).sum::<f64>() / total_licenses as f64
        } else {
            0.0
        };

        // Get unique ULOs
        let mut ulo_names: Vec<String> = licenses.iter().map(|l| l.ulo_name.clone()).collect();
        ulo_names.sort();
        ulo_names.dedup();

        Ok(AgentPerformance {
            agent_name: agent.name,
            agent_email: agent.email,
            country: agent.country,
            commission_percent: agent.commission_percent,
            total_licenses,
            online_licenses,
            avg_uptime,
            ulo_count: ulo_names.len(),
        })
    }
}

/// Parse CSV content and create NewLicense entries for an agent
/// Expected format: license_id,lease_code
pub fn parse_license_csv_for_agent(
    csv_content: &str,
    agent_id: &str,
) -> Result<Vec<NewLicense>, String> {
    let mut licenses = Vec::new();
    let lines: Vec<&str> = csv_content.lines().collect();

    if lines.is_empty() {
        return Err("Empty CSV file".to_string());
    }

    // Skip header line
    for (line_num, line) in lines.iter().enumerate().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 2 {
            return Err(format!(
                "Line {}: Expected 2 fields (license_id,lease_code), got {}",
                line_num + 1,
                fields.len()
            ));
        }

        let license_id = fields[0].trim().to_string();
        let lease_code = fields[1].trim().to_string();

        if license_id.is_empty() {
            return Err(format!("Line {}: license_id cannot be empty", line_num + 1));
        }

        // Create license with defaults - node_id and ulo_name will be populated from API sync
        let mut license = NewLicense::new(
            license_id,
            String::new(), // node_id - will be updated from API
            agent_id.to_string(),
            String::new(), // ulo_name - will be updated from API
        );

        if !lease_code.is_empty() {
            license = license.with_lease_code(lease_code);
        }

        licenses.push(license);
    }

    if licenses.is_empty() {
        return Err("No valid licenses found in CSV".to_string());
    }

    Ok(licenses)
}

/// Agent performance metrics
#[derive(Debug, Clone, Default)]
pub struct AgentPerformance {
    pub agent_name: String,
    pub agent_email: String,
    pub country: String,
    pub commission_percent: f64,
    pub total_licenses: usize,
    pub online_licenses: usize,
    pub avg_uptime: f64,
    pub ulo_count: usize,
}

impl AgentPerformance {
    pub fn online_percentage(&self) -> f64 {
        if self.total_licenses == 0 {
            0.0
        } else {
            (self.online_licenses as f64 / self.total_licenses as f64) * 100.0
        }
    }

    pub fn avg_uptime_percentage(&self) -> f64 {
        self.avg_uptime * 100.0
    }
}
