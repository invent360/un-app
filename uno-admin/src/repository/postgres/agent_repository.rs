//! PostgreSQL implementation of AgentRepositoryTrait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::models::entity::{AgentEntity, NewAgent};
use crate::repository::traits::AgentRepositoryTrait;

/// Database row structure for agents table
#[derive(sqlx::FromRow)]
struct AgentRow {
    id: String,
    name: String,
    email: String,
    country: String,
    commission_percent: f64,
    referral_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<AgentRow> for AgentEntity {
    fn from(row: AgentRow) -> Self {
        AgentEntity {
            id: row.id,
            name: row.name,
            email: row.email,
            country: row.country,
            commission_percent: row.commission_percent,
            referral_code: row.referral_code,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
        }
    }
}

/// PostgreSQL implementation of the Agent repository
pub struct PgAgentRepository {
    pool: Arc<PgPool>,
}

impl PgAgentRepository {
    /// Create a new PgAgentRepository with the given connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgentRepositoryTrait for PgAgentRepository {
    async fn save_agent(&self, new_agent: NewAgent) -> Result<AgentEntity, String> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        let row = sqlx::query_as::<_, AgentRow>(
            r#"
            INSERT INTO agents (
                id, name, email, country, commission_percent,
                referral_code, is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, name, email, country, commission_percent,
                      referral_code, created_at, updated_at
            "#,
        )
        .bind(&id)
        .bind(&new_agent.name)
        .bind(&new_agent.email)
        .bind(&new_agent.country)
        .bind(new_agent.commission_percent)
        .bind(&new_agent.referral_code)
        .bind(true) // is_active
        .bind(now)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to save agent: {}", e);
            e.to_string()
        })?;

        debug!("Saved agent: {}", id);

        Ok(row.into())
    }

    async fn get_agent_by_id(&self, id: &str) -> Result<Option<AgentEntity>, String> {
        let row = sqlx::query_as::<_, AgentRow>(
            r#"
            SELECT id, name, email, country, commission_percent,
                   referral_code, created_at, updated_at
            FROM agents
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get agent by id: {}", e);
            e.to_string()
        })?;

        Ok(row.map(Into::into))
    }

    async fn get_agent_by_email(&self, email: &str) -> Result<Option<AgentEntity>, String> {
        let row = sqlx::query_as::<_, AgentRow>(
            r#"
            SELECT id, name, email, country, commission_percent,
                   referral_code, created_at, updated_at
            FROM agents
            WHERE email = $1 AND is_active = true
            "#,
        )
        .bind(email)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get agent by email: {}", e);
            e.to_string()
        })?;

        Ok(row.map(Into::into))
    }

    async fn list_agents(&self) -> Result<Vec<AgentEntity>, String> {
        let rows = sqlx::query_as::<_, AgentRow>(
            r#"
            SELECT id, name, email, country, commission_percent,
                   referral_code, created_at, updated_at
            FROM agents
            WHERE is_active = true
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to list agents: {}", e);
            e.to_string()
        })?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_agent(&self, agent: AgentEntity) -> Result<AgentEntity, String> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, AgentRow>(
            r#"
            UPDATE agents
            SET name = $1,
                email = $2,
                country = $3,
                commission_percent = $4,
                referral_code = $5,
                updated_at = $6
            WHERE id = $7 AND is_active = true
            RETURNING id, name, email, country, commission_percent,
                      referral_code, created_at, updated_at
            "#,
        )
        .bind(&agent.name)
        .bind(&agent.email)
        .bind(&agent.country)
        .bind(agent.commission_percent)
        .bind(&agent.referral_code)
        .bind(now)
        .bind(&agent.id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to update agent: {}", e);
            e.to_string()
        })?;

        debug!("Updated agent: {}", agent.id);

        Ok(row.into())
    }

    async fn delete_agent(&self, id: &str) -> Result<bool, String> {
        let now = Utc::now();

        // Soft delete: set is_active = false and deleted_at timestamp
        let result = sqlx::query(
            r#"
            UPDATE agents
            SET is_active = false, deleted_at = $1, updated_at = $1
            WHERE id = $2 AND is_active = true
            "#,
        )
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to delete agent: {}", e);
            e.to_string()
        })?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            debug!("Soft deleted agent: {}", id);
        }

        Ok(deleted)
    }

    async fn count_agents(&self) -> Result<i64, String> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM agents
            WHERE is_active = true
            "#,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to count agents: {}", e);
            e.to_string()
        })?;

        Ok(count.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require a test database
    // These are placeholder tests for the module structure

    #[test]
    fn test_agent_row_to_entity_conversion() {
        let now = Utc::now();
        let row = AgentRow {
            id: "test-id".to_string(),
            name: "Test Agent".to_string(),
            email: "test@example.com".to_string(),
            country: "US".to_string(),
            commission_percent: 10.0,
            referral_code: Some("REF123".to_string()),
            created_at: now,
            updated_at: now,
        };

        let entity: AgentEntity = row.into();

        assert_eq!(entity.id, "test-id");
        assert_eq!(entity.name, "Test Agent");
        assert_eq!(entity.email, "test@example.com");
        assert_eq!(entity.country, "US");
        assert_eq!(entity.commission_percent, 10.0);
        assert_eq!(entity.referral_code, Some("REF123".to_string()));
    }
}
