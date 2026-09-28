//! Audit service for logging and querying CMS actions

use serde_json::json;
use crate::server::repositories::DynAuditRepository;
use crate::types::{
    AppError, AuditLogEntry, CreateAuditLog, AuditLogFilter, AuditLogResponse,
    audit::{actions, entities},
};

/// Audit service for tracking CMS actions
#[derive(Clone)]
pub struct AuditServiceImpl {
    audit_repo: DynAuditRepository,
}

impl AuditServiceImpl {
    pub fn new(audit_repo: DynAuditRepository) -> Self {
        Self { audit_repo }
    }

    /// Log a content action
    pub async fn log_content_action(
        &self,
        action: &str,
        content_id: i32,
        actor_id: &str,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
    ) -> Result<AuditLogEntry, AppError> {
        self.audit_repo.create(CreateAuditLog {
            entity_type: entities::CONTENT.to_string(),
            entity_id: Some(content_id),
            action: action.to_string(),
            actor_id: actor_id.to_string(),
            actor_name: None,
            old_values,
            new_values,
            metadata: None,
        }).await
    }

    /// Log content creation
    pub async fn log_content_created(
        &self,
        content_id: i32,
        content_type: &str,
        slug: &str,
        actor_id: &str,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::CREATE,
            content_id,
            actor_id,
            None,
            Some(json!({
                "content_type": content_type,
                "slug": slug,
            })),
        ).await
    }

    /// Log content update
    pub async fn log_content_updated(
        &self,
        content_id: i32,
        old_version: i32,
        new_version: i32,
        actor_id: &str,
        change_summary: Option<&str>,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::UPDATE,
            content_id,
            actor_id,
            Some(json!({ "version": old_version })),
            Some(json!({
                "version": new_version,
                "change_summary": change_summary,
            })),
        ).await
    }

    /// Log content publish
    pub async fn log_content_published(
        &self,
        content_id: i32,
        old_status: &str,
        version: i32,
        actor_id: &str,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::PUBLISH,
            content_id,
            actor_id,
            Some(json!({ "status": old_status })),
            Some(json!({
                "status": "published",
                "version": version,
            })),
        ).await
    }

    /// Log content archive/unpublish
    pub async fn log_content_archived(
        &self,
        content_id: i32,
        old_status: &str,
        actor_id: &str,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::ARCHIVE,
            content_id,
            actor_id,
            Some(json!({ "status": old_status })),
            Some(json!({ "status": "archived" })),
        ).await
    }

    /// Log content revert
    pub async fn log_content_reverted(
        &self,
        content_id: i32,
        old_version: i32,
        new_version: i32,
        reverted_to_version: i32,
        actor_id: &str,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::REVERT,
            content_id,
            actor_id,
            Some(json!({ "version": old_version })),
            Some(json!({
                "version": new_version,
                "reverted_to": reverted_to_version,
            })),
        ).await
    }

    /// Log content schedule update
    pub async fn log_content_scheduled(
        &self,
        content_id: i32,
        publish_at: Option<chrono::DateTime<chrono::Utc>>,
        unpublish_at: Option<chrono::DateTime<chrono::Utc>>,
        actor_id: &str,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::SCHEDULE,
            content_id,
            actor_id,
            None,
            Some(json!({
                "publish_at": publish_at,
                "unpublish_at": unpublish_at,
            })),
        ).await
    }

    /// Log content deletion
    pub async fn log_content_deleted(
        &self,
        content_id: i32,
        actor_id: &str,
    ) -> Result<AuditLogEntry, AppError> {
        self.log_content_action(
            actions::DELETE,
            content_id,
            actor_id,
            None,
            Some(json!({ "deleted": true })),
        ).await
    }

    /// List audit logs with filters
    pub async fn list_logs(&self, filter: AuditLogFilter) -> Result<AuditLogResponse, AppError> {
        self.audit_repo.list(filter).await
    }

    /// Get audit history for a specific content item
    pub async fn get_content_history(&self, content_id: i32) -> Result<Vec<AuditLogEntry>, AppError> {
        self.audit_repo.get_by_entity(entities::CONTENT, content_id).await
    }

    /// Get audit history for a specific entity
    pub async fn get_entity_history(&self, entity_type: &str, entity_id: i32) -> Result<Vec<AuditLogEntry>, AppError> {
        self.audit_repo.get_by_entity(entity_type, entity_id).await
    }
}
