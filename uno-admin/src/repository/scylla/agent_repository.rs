//! ScyllaDB implementation of AgentRepositoryTrait

use async_trait::async_trait;
use chrono::Utc;
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::entity::{AgentEntity, NewAgent};
use crate::repository::traits::AgentRepositoryTrait;

type AgentRow = (String, String, String, String, f64, Option<String>, CqlTimestamp, CqlTimestamp);

/// ScyllaDB implementation of the Agent repository
pub struct AgentRepository {
    session: Arc<Session>,
}

impl AgentRepository {
    /// Create a new AgentRepository with the given session
    pub fn new(session: DbPool) -> Self {
        Self { session }
    }

    fn timestamp_to_datetime(ts: CqlTimestamp) -> String {
        chrono::DateTime::from_timestamp_millis(ts.0)
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }

    fn parse_single_agent(result: scylla::QueryResult) -> Result<Option<AgentEntity>, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<AgentRow>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        for row_result in rows {
            if let Ok((id, name, email, country, commission_percent, referral_code, created_at, updated_at)) = row_result {
                return Ok(Some(AgentEntity {
                    id,
                    name,
                    email,
                    country,
                    commission_percent,
                    referral_code,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                }));
            }
        }

        Ok(None)
    }

    fn parse_agents(result: scylla::QueryResult) -> Result<Vec<AgentEntity>, String> {
        let mut agents = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(agents),
        };

        let rows = match rows_result.rows::<AgentRow>() {
            Ok(r) => r,
            Err(_) => return Ok(agents),
        };

        for row_result in rows {
            if let Ok((id, name, email, country, commission_percent, referral_code, created_at, updated_at)) = row_result {
                agents.push(AgentEntity {
                    id,
                    name,
                    email,
                    country,
                    commission_percent,
                    referral_code,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                });
            }
        }

        Ok(agents)
    }
}

#[async_trait]
impl AgentRepositoryTrait for AgentRepository {
    async fn save_agent(&self, new_agent: NewAgent) -> Result<AgentEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());
        let id = Uuid::new_v4().to_string();

        let query = "INSERT INTO agents (id, name, email, country, commission_percent, referral_code, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.session
            .query_unpaged(
                query,
                (
                    &id,
                    &new_agent.name,
                    &new_agent.email,
                    &new_agent.country,
                    new_agent.commission_percent,
                    &new_agent.referral_code,
                    true,
                    now_ts,
                    now_ts,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to save agent: {}", e);
                e.to_string()
            })?;

        debug!("Saved agent: {}", id);

        Ok(AgentEntity {
            id,
            name: new_agent.name,
            email: new_agent.email,
            country: new_agent.country,
            commission_percent: new_agent.commission_percent,
            referral_code: new_agent.referral_code,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
        })
    }

    async fn get_agent_by_id(&self, id: &str) -> Result<Option<AgentEntity>, String> {
        let query = "SELECT id, name, email, country, commission_percent, referral_code, created_at, updated_at FROM agents WHERE id = ?";

        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_single_agent(result)
    }

    async fn get_agent_by_email(&self, email: &str) -> Result<Option<AgentEntity>, String> {
        let query = "SELECT id, name, email, country, commission_percent, referral_code, created_at, updated_at FROM agents WHERE email = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (email,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_single_agent(result)
    }

    async fn list_agents(&self) -> Result<Vec<AgentEntity>, String> {
        let query = "SELECT id, name, email, country, commission_percent, referral_code, created_at, updated_at FROM agents WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_agents(result)
    }

    async fn update_agent(&self, agent: AgentEntity) -> Result<AgentEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());

        let query = "UPDATE agents SET name = ?, email = ?, country = ?, commission_percent = ?, referral_code = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    &agent.name,
                    &agent.email,
                    &agent.country,
                    agent.commission_percent,
                    &agent.referral_code,
                    now_ts,
                    &agent.id,
                ),
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(AgentEntity {
            updated_at: now.to_rfc3339(),
            ..agent
        })
    }

    async fn delete_agent(&self, id: &str) -> Result<bool, String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        // Soft delete
        let query = "UPDATE agents SET is_active = false, deleted_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, (now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }

    async fn count_agents(&self) -> Result<i64, String> {
        let query = "SELECT COUNT(*) FROM agents WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let rows = match rows_result.rows::<(i64,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        for row_result in rows {
            if let Ok((count,)) = row_result {
                return Ok(count);
            }
        }

        Ok(0)
    }
}
