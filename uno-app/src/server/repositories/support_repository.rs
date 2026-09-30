//! Support repository for ticket management
//!
//! Implements CRUD operations for:
//! - Support tickets (create, update, assign, resolve)
//! - Ticket messages (conversation thread)
//! - Ticket history (audit trail)
//! - Queue management (agent assignments)

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENUMS (matching PostgreSQL types)
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Open,
    InProgress,
    WaitingUser,
    Resolved,
    Closed,
}

impl TicketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketStatus::Open => "open",
            TicketStatus::InProgress => "in_progress",
            TicketStatus::WaitingUser => "waiting_user",
            TicketStatus::Resolved => "resolved",
            TicketStatus::Closed => "closed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "open" => Some(TicketStatus::Open),
            "in_progress" => Some(TicketStatus::InProgress),
            "waiting_user" => Some(TicketStatus::WaitingUser),
            "resolved" => Some(TicketStatus::Resolved),
            "closed" => Some(TicketStatus::Closed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl TicketPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketPriority::Low => "low",
            TicketPriority::Normal => "normal",
            TicketPriority::High => "high",
            TicketPriority::Urgent => "urgent",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "low" => Some(TicketPriority::Low),
            "normal" => Some(TicketPriority::Normal),
            "high" => Some(TicketPriority::High),
            "urgent" => Some(TicketPriority::Urgent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketCategory {
    Account,
    Technical,
    Billing,
    Claim,
    Agent,
    Compliance,
    Other,
}

impl TicketCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketCategory::Account => "account",
            TicketCategory::Technical => "technical",
            TicketCategory::Billing => "billing",
            TicketCategory::Claim => "claim",
            TicketCategory::Agent => "agent",
            TicketCategory::Compliance => "compliance",
            TicketCategory::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "account" => Some(TicketCategory::Account),
            "technical" => Some(TicketCategory::Technical),
            "billing" => Some(TicketCategory::Billing),
            "claim" => Some(TicketCategory::Claim),
            "agent" => Some(TicketCategory::Agent),
            "compliance" => Some(TicketCategory::Compliance),
            "other" => Some(TicketCategory::Other),
            _ => None,
        }
    }
}

// ============================================
// ENTITY STRUCTS
// ============================================

/// Support ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: Uuid,
    pub ticket_number: String,
    pub user_id: String,
    pub license_id: Option<String>,
    pub category: TicketCategory,
    pub priority: TicketPriority,
    pub status: TicketStatus,
    pub subject: String,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub country_code: Option<String>,
    pub assigned_agent_id: Option<Uuid>,
    pub escalated_at: Option<DateTime<Utc>>,
    pub escalation_reason: Option<String>,
    pub escalation_level: i32,
    pub first_response_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub satisfaction_score: Option<i32>,
    pub satisfaction_feedback: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct SupportTicketRow {
    id: Uuid,
    ticket_number: String,
    user_id: String,
    license_id: Option<String>,
    category: String,
    priority: String,
    status: String,
    subject: String,
    description: Option<String>,
    locale: Option<String>,
    country_code: Option<String>,
    assigned_agent_id: Option<Uuid>,
    escalated_at: Option<DateTime<Utc>>,
    escalation_reason: Option<String>,
    escalation_level: i32,
    first_response_at: Option<DateTime<Utc>>,
    resolved_at: Option<DateTime<Utc>>,
    resolution_notes: Option<String>,
    satisfaction_score: Option<i32>,
    satisfaction_feedback: Option<String>,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<SupportTicketRow> for SupportTicket {
    fn from(row: SupportTicketRow) -> Self {
        SupportTicket {
            id: row.id,
            ticket_number: row.ticket_number,
            user_id: row.user_id,
            license_id: row.license_id,
            category: TicketCategory::from_str(&row.category).unwrap_or(TicketCategory::Other),
            priority: TicketPriority::from_str(&row.priority).unwrap_or(TicketPriority::Normal),
            status: TicketStatus::from_str(&row.status).unwrap_or(TicketStatus::Open),
            subject: row.subject,
            description: row.description,
            locale: row.locale,
            country_code: row.country_code,
            assigned_agent_id: row.assigned_agent_id,
            escalated_at: row.escalated_at,
            escalation_reason: row.escalation_reason,
            escalation_level: row.escalation_level,
            first_response_at: row.first_response_at,
            resolved_at: row.resolved_at,
            resolution_notes: row.resolution_notes,
            satisfaction_score: row.satisfaction_score,
            satisfaction_feedback: row.satisfaction_feedback,
            tags: row.tags,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Ticket message in conversation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TicketMessage {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_type: String,
    pub sender_id: String,
    pub message: String,
    pub is_internal: bool,
    pub attachments: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Ticket history entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TicketHistory {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub field_changed: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: String,
    pub changed_by_type: String,
    pub changed_at: DateTime<Utc>,
}

/// Queue assignment for agents
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QueueAssignment {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub category: String,
    pub country_code: Option<String>,
    pub locale: Option<String>,
    pub max_concurrent: i32,
    pub current_load: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Canned response for agents
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CannedResponse {
    pub id: Uuid,
    pub category: String,
    pub title: String,
    pub content: String,
    pub locale: String,
    pub shortcut: Option<String>,
    pub usage_count: i32,
    pub created_by: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating a new ticket
#[derive(Debug, Clone)]
pub struct CreateTicketInput {
    pub user_id: String,
    pub license_id: Option<String>,
    pub category: TicketCategory,
    pub priority: TicketPriority,
    pub subject: String,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub country_code: Option<String>,
    pub tags: Vec<String>,
}

/// Input for adding a message to a ticket
#[derive(Debug, Clone)]
pub struct AddMessageInput {
    pub ticket_id: Uuid,
    pub sender_type: String,
    pub sender_id: String,
    pub message: String,
    pub is_internal: bool,
    pub attachments: Option<serde_json::Value>,
}

/// Filters for searching tickets
#[derive(Debug, Clone, Default)]
pub struct TicketFilters {
    pub user_id: Option<String>,
    pub agent_id: Option<Uuid>,
    pub status: Option<TicketStatus>,
    pub category: Option<TicketCategory>,
    pub priority: Option<TicketPriority>,
    pub country_code: Option<String>,
    pub unassigned_only: bool,
}

/// Queue statistics
#[derive(Debug, Clone, Serialize)]
pub struct QueueStats {
    pub total_open: i64,
    pub total_in_progress: i64,
    pub total_unassigned: i64,
    pub by_category: Vec<CategoryCount>,
    pub by_priority: Vec<PriorityCount>,
    pub avg_first_response_hours: Option<f64>,
    pub avg_resolution_hours: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PriorityCount {
    pub priority: String,
    pub count: i64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for SupportRepository trait object
pub type DynSupportRepository = Arc<dyn SupportRepository + Send + Sync>;

/// Support repository trait defining database operations
#[async_trait]
pub trait SupportRepository: Send + Sync {
    // --- Ticket Operations ---

    /// Create a new support ticket
    async fn create_ticket(&self, input: CreateTicketInput) -> Result<SupportTicket, AppError>;

    /// Get ticket by ID
    async fn get_ticket(&self, id: Uuid) -> Result<Option<SupportTicket>, AppError>;

    /// Get ticket by ticket number
    async fn get_ticket_by_number(&self, ticket_number: &str) -> Result<Option<SupportTicket>, AppError>;

    /// Search tickets with filters
    async fn search_tickets(
        &self,
        filters: &TicketFilters,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<SupportTicket>, AppError>;

    /// Count tickets matching filters
    async fn count_tickets(&self, filters: &TicketFilters) -> Result<i64, AppError>;

    /// Update ticket status
    async fn update_status(
        &self,
        id: Uuid,
        status: TicketStatus,
        changed_by: &str,
        changed_by_type: &str,
    ) -> Result<SupportTicket, AppError>;

    /// Assign ticket to agent
    async fn assign_ticket(
        &self,
        id: Uuid,
        agent_id: Uuid,
        changed_by: &str,
    ) -> Result<SupportTicket, AppError>;

    /// Escalate ticket
    async fn escalate_ticket(
        &self,
        id: Uuid,
        reason: &str,
        escalated_by: &str,
    ) -> Result<SupportTicket, AppError>;

    /// Resolve ticket
    async fn resolve_ticket(
        &self,
        id: Uuid,
        resolution_notes: &str,
        resolved_by: &str,
    ) -> Result<SupportTicket, AppError>;

    /// Record satisfaction feedback
    async fn record_satisfaction(
        &self,
        id: Uuid,
        score: i32,
        feedback: Option<&str>,
    ) -> Result<SupportTicket, AppError>;

    // --- Message Operations ---

    /// Add message to ticket
    async fn add_message(&self, input: AddMessageInput) -> Result<TicketMessage, AppError>;

    /// Get messages for ticket
    async fn get_messages(&self, ticket_id: Uuid, include_internal: bool) -> Result<Vec<TicketMessage>, AppError>;

    // --- History Operations ---

    /// Get ticket history
    async fn get_history(&self, ticket_id: Uuid) -> Result<Vec<TicketHistory>, AppError>;

    // --- Queue Operations ---

    /// Get agent's queue
    async fn get_agent_queue(&self, agent_id: Uuid) -> Result<Vec<SupportTicket>, AppError>;

    /// Get unassigned tickets for category/country
    async fn get_unassigned_tickets(
        &self,
        category: Option<TicketCategory>,
        country_code: Option<&str>,
        limit: i32,
    ) -> Result<Vec<SupportTicket>, AppError>;

    /// Auto-assign ticket to available agent
    async fn auto_assign_ticket(&self, ticket_id: Uuid) -> Result<Option<Uuid>, AppError>;

    /// Get queue statistics
    async fn get_queue_stats(&self, country_code: Option<&str>) -> Result<QueueStats, AppError>;

    // --- Canned Response Operations ---

    /// Get canned responses for category
    async fn get_canned_responses(
        &self,
        category: TicketCategory,
        locale: &str,
    ) -> Result<Vec<CannedResponse>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct SupportRepositoryImpl {
    pool: ConnectionPool,
}

impl SupportRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SupportRepository for SupportRepositoryImpl {
    async fn create_ticket(&self, input: CreateTicketInput) -> Result<SupportTicket, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            INSERT INTO support_tickets (
                user_id, license_id, category, priority, subject, description,
                locale, country_code, tags
            )
            VALUES ($1, $2, $3::ticket_category, $4::ticket_priority, $5, $6, $7, $8, $9)
            RETURNING id, ticket_number, user_id, license_id, category::text, priority::text,
                      status::text, subject, description, locale, country_code,
                      assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                      first_response_at, resolved_at, resolution_notes,
                      satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.license_id)
        .bind(input.category.as_str())
        .bind(input.priority.as_str())
        .bind(&input.subject)
        .bind(&input.description)
        .bind(&input.locale)
        .bind(&input.country_code)
        .bind(&input.tags)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportTicket::from(row))
    }

    async fn get_ticket(&self, id: Uuid) -> Result<Option<SupportTicket>, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                   status::text, subject, description, locale, country_code,
                   assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                   first_response_at, resolved_at, resolution_notes,
                   satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            FROM support_tickets
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(SupportTicket::from))
    }

    async fn get_ticket_by_number(&self, ticket_number: &str) -> Result<Option<SupportTicket>, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                   status::text, subject, description, locale, country_code,
                   assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                   first_response_at, resolved_at, resolution_notes,
                   satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            FROM support_tickets
            WHERE ticket_number = $1
            "#,
        )
        .bind(ticket_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(SupportTicket::from))
    }

    async fn search_tickets(
        &self,
        filters: &TicketFilters,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<SupportTicket>, AppError> {
        let mut query = String::from(
            r#"
            SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                   status::text, subject, description, locale, country_code,
                   assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                   first_response_at, resolved_at, resolution_notes,
                   satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            FROM support_tickets
            WHERE 1=1
            "#,
        );

        let mut param_idx = 1;
        let mut conditions = Vec::new();

        if filters.user_id.is_some() {
            conditions.push(format!("user_id = ${}", param_idx));
            param_idx += 1;
        }
        if filters.agent_id.is_some() {
            conditions.push(format!("assigned_agent_id = ${}", param_idx));
            param_idx += 1;
        }
        if filters.status.is_some() {
            conditions.push(format!("status = ${}::ticket_status", param_idx));
            param_idx += 1;
        }
        if filters.category.is_some() {
            conditions.push(format!("category = ${}::ticket_category", param_idx));
            param_idx += 1;
        }
        if filters.priority.is_some() {
            conditions.push(format!("priority = ${}::ticket_priority", param_idx));
            param_idx += 1;
        }
        if filters.country_code.is_some() {
            conditions.push(format!("country_code = ${}", param_idx));
            param_idx += 1;
        }
        if filters.unassigned_only {
            conditions.push("assigned_agent_id IS NULL".to_string());
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(&format!(
            " ORDER BY CASE priority WHEN 'urgent' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END, created_at ASC LIMIT ${} OFFSET ${}",
            param_idx,
            param_idx + 1
        ));

        let mut q = sqlx::query_as::<_, SupportTicketRow>(&query);

        if let Some(ref user_id) = filters.user_id {
            q = q.bind(user_id);
        }
        if let Some(agent_id) = filters.agent_id {
            q = q.bind(agent_id);
        }
        if let Some(status) = filters.status {
            q = q.bind(status.as_str());
        }
        if let Some(category) = filters.category {
            q = q.bind(category.as_str());
        }
        if let Some(priority) = filters.priority {
            q = q.bind(priority.as_str());
        }
        if let Some(ref country_code) = filters.country_code {
            q = q.bind(country_code);
        }

        q = q.bind(limit).bind(offset);

        let rows = q
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(SupportTicket::from).collect())
    }

    async fn count_tickets(&self, filters: &TicketFilters) -> Result<i64, AppError> {
        let mut query = String::from("SELECT COUNT(*) FROM support_tickets WHERE 1=1");

        let mut conditions = Vec::new();
        let mut param_idx = 1;

        if filters.user_id.is_some() {
            conditions.push(format!("user_id = ${}", param_idx));
            param_idx += 1;
        }
        if filters.agent_id.is_some() {
            conditions.push(format!("assigned_agent_id = ${}", param_idx));
            param_idx += 1;
        }
        if filters.status.is_some() {
            conditions.push(format!("status = ${}::ticket_status", param_idx));
            param_idx += 1;
        }
        if filters.category.is_some() {
            conditions.push(format!("category = ${}::ticket_category", param_idx));
            param_idx += 1;
        }
        if filters.priority.is_some() {
            conditions.push(format!("priority = ${}::ticket_priority", param_idx));
            // param_idx += 1;
        }
        if filters.country_code.is_some() {
            conditions.push(format!("country_code = ${}", param_idx));
            // param_idx += 1;
        }
        if filters.unassigned_only {
            conditions.push("assigned_agent_id IS NULL".to_string());
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        let mut q = sqlx::query_as::<_, (i64,)>(&query);

        if let Some(ref user_id) = filters.user_id {
            q = q.bind(user_id);
        }
        if let Some(agent_id) = filters.agent_id {
            q = q.bind(agent_id);
        }
        if let Some(status) = filters.status {
            q = q.bind(status.as_str());
        }
        if let Some(category) = filters.category {
            q = q.bind(category.as_str());
        }
        if let Some(priority) = filters.priority {
            q = q.bind(priority.as_str());
        }
        if let Some(ref country_code) = filters.country_code {
            q = q.bind(country_code);
        }

        let (count,) = q
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: TicketStatus,
        changed_by: &str,
        changed_by_type: &str,
    ) -> Result<SupportTicket, AppError> {
        // Get current status for history
        let current = self.get_ticket(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", id)))?;

        // Update status
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            UPDATE support_tickets
            SET status = $2::ticket_status, updated_at = NOW()
            WHERE id = $1
            RETURNING id, ticket_number, user_id, license_id, category::text, priority::text,
                      status::text, subject, description, locale, country_code,
                      assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                      first_response_at, resolved_at, resolution_notes,
                      satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Record history
        sqlx::query(
            r#"
            INSERT INTO support_ticket_history (ticket_id, field_changed, old_value, new_value, changed_by, changed_by_type)
            VALUES ($1, 'status', $2, $3, $4, $5)
            "#,
        )
        .bind(id)
        .bind(current.status.as_str())
        .bind(status.as_str())
        .bind(changed_by)
        .bind(changed_by_type)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportTicket::from(row))
    }

    async fn assign_ticket(
        &self,
        id: Uuid,
        agent_id: Uuid,
        changed_by: &str,
    ) -> Result<SupportTicket, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            UPDATE support_tickets
            SET assigned_agent_id = $2,
                status = CASE WHEN status = 'open'::ticket_status THEN 'in_progress'::ticket_status ELSE status END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, ticket_number, user_id, license_id, category::text, priority::text,
                      status::text, subject, description, locale, country_code,
                      assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                      first_response_at, resolved_at, resolution_notes,
                      satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(agent_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Record history
        sqlx::query(
            r#"
            INSERT INTO support_ticket_history (ticket_id, field_changed, new_value, changed_by, changed_by_type)
            VALUES ($1, 'assigned_agent_id', $2::text, $3, 'agent')
            "#,
        )
        .bind(id)
        .bind(agent_id)
        .bind(changed_by)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportTicket::from(row))
    }

    async fn escalate_ticket(
        &self,
        id: Uuid,
        reason: &str,
        escalated_by: &str,
    ) -> Result<SupportTicket, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            UPDATE support_tickets
            SET escalated_at = NOW(),
                escalation_reason = $2,
                escalation_level = escalation_level + 1,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, ticket_number, user_id, license_id, category::text, priority::text,
                      status::text, subject, description, locale, country_code,
                      assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                      first_response_at, resolved_at, resolution_notes,
                      satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Record history
        sqlx::query(
            r#"
            INSERT INTO support_ticket_history (ticket_id, field_changed, new_value, changed_by, changed_by_type)
            VALUES ($1, 'escalated', $2, $3, 'agent')
            "#,
        )
        .bind(id)
        .bind(reason)
        .bind(escalated_by)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportTicket::from(row))
    }

    async fn resolve_ticket(
        &self,
        id: Uuid,
        resolution_notes: &str,
        resolved_by: &str,
    ) -> Result<SupportTicket, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            UPDATE support_tickets
            SET status = 'resolved'::ticket_status,
                resolved_at = NOW(),
                resolution_notes = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, ticket_number, user_id, license_id, category::text, priority::text,
                      status::text, subject, description, locale, country_code,
                      assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                      first_response_at, resolved_at, resolution_notes,
                      satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(resolution_notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Record history
        sqlx::query(
            r#"
            INSERT INTO support_ticket_history (ticket_id, field_changed, new_value, changed_by, changed_by_type)
            VALUES ($1, 'resolved', $2, $3, 'agent')
            "#,
        )
        .bind(id)
        .bind(resolution_notes)
        .bind(resolved_by)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportTicket::from(row))
    }

    async fn record_satisfaction(
        &self,
        id: Uuid,
        score: i32,
        feedback: Option<&str>,
    ) -> Result<SupportTicket, AppError> {
        let row = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            UPDATE support_tickets
            SET satisfaction_score = $2,
                satisfaction_feedback = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, ticket_number, user_id, license_id, category::text, priority::text,
                      status::text, subject, description, locale, country_code,
                      assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                      first_response_at, resolved_at, resolution_notes,
                      satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(score)
        .bind(feedback)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportTicket::from(row))
    }

    async fn add_message(&self, input: AddMessageInput) -> Result<TicketMessage, AppError> {
        // Record first response time if agent is responding and no first_response_at yet
        if input.sender_type == "agent" {
            sqlx::query(
                r#"
                UPDATE support_tickets
                SET first_response_at = COALESCE(first_response_at, NOW()),
                    updated_at = NOW()
                WHERE id = $1
                "#,
            )
            .bind(input.ticket_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        let message = sqlx::query_as::<_, TicketMessage>(
            r#"
            INSERT INTO support_ticket_messages (ticket_id, sender_type, sender_id, message, is_internal, attachments)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, ticket_id, sender_type, sender_id, message, is_internal, attachments, created_at
            "#,
        )
        .bind(input.ticket_id)
        .bind(&input.sender_type)
        .bind(&input.sender_id)
        .bind(&input.message)
        .bind(input.is_internal)
        .bind(&input.attachments)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn get_messages(&self, ticket_id: Uuid, include_internal: bool) -> Result<Vec<TicketMessage>, AppError> {
        let messages = if include_internal {
            sqlx::query_as::<_, TicketMessage>(
                r#"
                SELECT id, ticket_id, sender_type, sender_id, message, is_internal, attachments, created_at
                FROM support_ticket_messages
                WHERE ticket_id = $1
                ORDER BY created_at ASC
                "#,
            )
            .bind(ticket_id)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, TicketMessage>(
                r#"
                SELECT id, ticket_id, sender_type, sender_id, message, is_internal, attachments, created_at
                FROM support_ticket_messages
                WHERE ticket_id = $1 AND is_internal = FALSE
                ORDER BY created_at ASC
                "#,
            )
            .bind(ticket_id)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(messages)
    }

    async fn get_history(&self, ticket_id: Uuid) -> Result<Vec<TicketHistory>, AppError> {
        let history = sqlx::query_as::<_, TicketHistory>(
            r#"
            SELECT id, ticket_id, field_changed, old_value, new_value, changed_by, changed_by_type, changed_at
            FROM support_ticket_history
            WHERE ticket_id = $1
            ORDER BY changed_at DESC
            "#,
        )
        .bind(ticket_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(history)
    }

    async fn get_agent_queue(&self, agent_id: Uuid) -> Result<Vec<SupportTicket>, AppError> {
        let rows = sqlx::query_as::<_, SupportTicketRow>(
            r#"
            SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                   status::text, subject, description, locale, country_code,
                   assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                   first_response_at, resolved_at, resolution_notes,
                   satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
            FROM support_tickets
            WHERE assigned_agent_id = $1
              AND status IN ('open', 'in_progress', 'waiting_user')
            ORDER BY CASE priority WHEN 'urgent' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END,
                     created_at ASC
            "#,
        )
        .bind(agent_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(SupportTicket::from).collect())
    }

    async fn get_unassigned_tickets(
        &self,
        category: Option<TicketCategory>,
        country_code: Option<&str>,
        limit: i32,
    ) -> Result<Vec<SupportTicket>, AppError> {
        let rows = if let Some(cat) = category {
            if let Some(cc) = country_code {
                sqlx::query_as::<_, SupportTicketRow>(
                    r#"
                    SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                           status::text, subject, description, locale, country_code,
                           assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                           first_response_at, resolved_at, resolution_notes,
                           satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
                    FROM support_tickets
                    WHERE assigned_agent_id IS NULL
                      AND status = 'open'
                      AND category = $1::ticket_category
                      AND country_code = $2
                    ORDER BY CASE priority WHEN 'urgent' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END,
                             created_at ASC
                    LIMIT $3
                    "#,
                )
                .bind(cat.as_str())
                .bind(cc)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            } else {
                sqlx::query_as::<_, SupportTicketRow>(
                    r#"
                    SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                           status::text, subject, description, locale, country_code,
                           assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                           first_response_at, resolved_at, resolution_notes,
                           satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
                    FROM support_tickets
                    WHERE assigned_agent_id IS NULL
                      AND status = 'open'
                      AND category = $1::ticket_category
                    ORDER BY CASE priority WHEN 'urgent' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END,
                             created_at ASC
                    LIMIT $2
                    "#,
                )
                .bind(cat.as_str())
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
        } else if let Some(cc) = country_code {
            sqlx::query_as::<_, SupportTicketRow>(
                r#"
                SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                       status::text, subject, description, locale, country_code,
                       assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                       first_response_at, resolved_at, resolution_notes,
                       satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
                FROM support_tickets
                WHERE assigned_agent_id IS NULL
                  AND status = 'open'
                  AND country_code = $1
                ORDER BY CASE priority WHEN 'urgent' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END,
                         created_at ASC
                LIMIT $2
                "#,
            )
            .bind(cc)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, SupportTicketRow>(
                r#"
                SELECT id, ticket_number, user_id, license_id, category::text, priority::text,
                       status::text, subject, description, locale, country_code,
                       assigned_agent_id, escalated_at, escalation_reason, escalation_level,
                       first_response_at, resolved_at, resolution_notes,
                       satisfaction_score, satisfaction_feedback, tags, created_at, updated_at
                FROM support_tickets
                WHERE assigned_agent_id IS NULL
                  AND status = 'open'
                ORDER BY CASE priority WHEN 'urgent' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END,
                         created_at ASC
                LIMIT $1
                "#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(SupportTicket::from).collect())
    }

    async fn auto_assign_ticket(&self, ticket_id: Uuid) -> Result<Option<Uuid>, AppError> {
        // Use the database function for auto-assignment
        let result = sqlx::query_as::<_, (Option<Uuid>,)>(
            "SELECT auto_assign_ticket($1)",
        )
        .bind(ticket_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    async fn get_queue_stats(&self, country_code: Option<&str>) -> Result<QueueStats, AppError> {
        let base_filter = country_code
            .map(|cc| format!("AND country_code = '{}'", cc))
            .unwrap_or_default();

        // Count by status
        let (total_open,): (i64,) = sqlx::query_as(&format!(
            "SELECT COUNT(*) FROM support_tickets WHERE status = 'open' {}",
            base_filter
        ))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (total_in_progress,): (i64,) = sqlx::query_as(&format!(
            "SELECT COUNT(*) FROM support_tickets WHERE status = 'in_progress' {}",
            base_filter
        ))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (total_unassigned,): (i64,) = sqlx::query_as(&format!(
            "SELECT COUNT(*) FROM support_tickets WHERE assigned_agent_id IS NULL AND status = 'open' {}",
            base_filter
        ))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Count by category
        let by_category: Vec<(String, i64)> = sqlx::query_as(&format!(
            "SELECT category::text, COUNT(*) FROM support_tickets WHERE status IN ('open', 'in_progress') {} GROUP BY category",
            base_filter
        ))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Count by priority
        let by_priority: Vec<(String, i64)> = sqlx::query_as(&format!(
            "SELECT priority::text, COUNT(*) FROM support_tickets WHERE status IN ('open', 'in_progress') {} GROUP BY priority",
            base_filter
        ))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Average response/resolution times
        let (avg_first_response_hours,): (Option<f64>,) = sqlx::query_as(&format!(
            r#"
            SELECT AVG(EXTRACT(EPOCH FROM (first_response_at - created_at)) / 3600)
            FROM support_tickets
            WHERE first_response_at IS NOT NULL {}
            "#,
            base_filter
        ))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (avg_resolution_hours,): (Option<f64>,) = sqlx::query_as(&format!(
            r#"
            SELECT AVG(EXTRACT(EPOCH FROM (resolved_at - created_at)) / 3600)
            FROM support_tickets
            WHERE resolved_at IS NOT NULL {}
            "#,
            base_filter
        ))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(QueueStats {
            total_open,
            total_in_progress,
            total_unassigned,
            by_category: by_category
                .into_iter()
                .map(|(cat, count)| CategoryCount {
                    category: cat,
                    count,
                })
                .collect(),
            by_priority: by_priority
                .into_iter()
                .map(|(pri, count)| PriorityCount {
                    priority: pri,
                    count,
                })
                .collect(),
            avg_first_response_hours,
            avg_resolution_hours,
        })
    }

    async fn get_canned_responses(
        &self,
        category: TicketCategory,
        locale: &str,
    ) -> Result<Vec<CannedResponse>, AppError> {
        let responses = sqlx::query_as::<_, CannedResponse>(
            r#"
            SELECT id, category, title, content, locale, shortcut, usage_count, created_by, is_active, created_at, updated_at
            FROM support_canned_responses
            WHERE category = $1 AND locale = $2 AND is_active = TRUE
            ORDER BY usage_count DESC
            "#,
        )
        .bind(category.as_str())
        .bind(locale)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(responses)
    }
}
