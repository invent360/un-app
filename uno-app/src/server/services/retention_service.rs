//! R3-17: Data Retention Service
//!
//! Enforces data retention policies by:
//! 1. Reading policies from the retention_policies table
//! 2. Archiving data before deletion (when configured)
//! 3. Deleting expired data in batches
//! 4. Logging all cleanup operations to the audit trail
//!
//! This service is designed to be run periodically by a background worker.

use std::sync::Arc;
use std::time::Instant;

use crate::server::repositories::{
    DynRetentionRepository, DynImmutableAuditRepository,
    RetentionPolicy, RetentionCleanupResult,
    AuditEvent, AuditEventType, AuditCategory, ActorType, AuditOutcome,
};
use crate::types::AppError;

// ============================================
// TYPES
// ============================================

/// Dynamic type alias for dependency injection
pub type DynRetentionService = Arc<dyn RetentionService + Send + Sync>;

/// Summary of a full cleanup run
#[derive(Debug, Clone, Default)]
pub struct CleanupSummary {
    pub policies_processed: usize,
    pub total_rows_deleted: i64,
    pub total_rows_archived: i64,
    pub total_duration_ms: u64,
    pub errors: Vec<String>,
    pub results: Vec<RetentionCleanupResult>,
}

impl CleanupSummary {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

// ============================================
// TRAIT
// ============================================

#[async_trait::async_trait]
pub trait RetentionService: Send + Sync {
    /// Run cleanup for all retention policies
    /// Returns a summary of what was cleaned up
    async fn run_full_cleanup(&self) -> Result<CleanupSummary, AppError>;

    /// Run cleanup for a specific table's policy
    async fn run_cleanup_for_table(&self, table_name: &str) -> Result<RetentionCleanupResult, AppError>;

    /// Get all policies with their last cleanup status
    async fn get_policy_status(&self) -> Result<Vec<RetentionPolicy>, AppError>;

    /// Check if any policies are overdue for cleanup
    async fn has_overdue_policies(&self, max_age_hours: i64) -> Result<bool, AppError>;
}

// ============================================
// IMPLEMENTATION
// ============================================

pub struct RetentionServiceImpl {
    retention_repo: DynRetentionRepository,
    audit_repo: DynImmutableAuditRepository,
}

impl RetentionServiceImpl {
    pub fn new(
        retention_repo: DynRetentionRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self {
            retention_repo,
            audit_repo,
        }
    }

    async fn log_cleanup_event(
        &self,
        policy: &RetentionPolicy,
        result: &RetentionCleanupResult,
    ) -> Result<(), AppError> {
        let outcome = if result.error.is_some() {
            AuditOutcome::Failure
        } else {
            AuditOutcome::Success
        };

        let event = AuditEvent::builder(
            AuditEventType::Custom("retention.cleanup".to_string()),
            AuditCategory::System,
        )
        .actor(ActorType::System, "retention_service")
        .resource("retention_policies", &policy.table_name)
        .action("cleanup")
        .outcome(outcome)
        .event_data(serde_json::json!({
            "table_name": policy.table_name,
            "retention_days": policy.retention_days,
            "rows_deleted": result.rows_deleted,
            "rows_archived": result.rows_archived,
            "duration_ms": result.duration_ms,
            "error": result.error,
        }))
        .build();

        self.audit_repo.log_event(event).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl RetentionService for RetentionServiceImpl {
    async fn run_full_cleanup(&self) -> Result<CleanupSummary, AppError> {
        let policies = self.retention_repo.list_policies().await?;
        let mut summary = CleanupSummary::default();

        let start = Instant::now();

        for policy in policies {
            // Skip policies with indefinite retention
            if policy.deletion_condition.is_empty()
                || policy.deletion_condition.to_uppercase() == "FALSE"
            {
                continue;
            }

            let result = self.run_cleanup_for_table(&policy.table_name).await?;

            summary.total_rows_deleted += result.rows_deleted;
            summary.total_rows_archived += result.rows_archived;

            if let Some(ref err) = result.error {
                summary.errors.push(format!("{}: {}", policy.table_name, err));
            }

            summary.results.push(result);
            summary.policies_processed += 1;
        }

        summary.total_duration_ms = start.elapsed().as_millis() as u64;

        // Log summary event
        let event = AuditEvent::builder(
            AuditEventType::Custom("retention.full_cleanup".to_string()),
            AuditCategory::System,
        )
        .actor(ActorType::System, "retention_service")
        .resource("retention_policies", "all")
        .action("full_cleanup")
        .outcome(if summary.has_errors() { AuditOutcome::Error } else { AuditOutcome::Success })
        .event_data(serde_json::json!({
            "policies_processed": summary.policies_processed,
            "total_rows_deleted": summary.total_rows_deleted,
            "total_rows_archived": summary.total_rows_archived,
            "total_duration_ms": summary.total_duration_ms,
            "errors_count": summary.errors.len(),
        }))
        .build();

        self.audit_repo.log_event(event).await?;

        Ok(summary)
    }

    async fn run_cleanup_for_table(&self, table_name: &str) -> Result<RetentionCleanupResult, AppError> {
        let policy = self.retention_repo.get_policy(table_name).await?
            .ok_or_else(|| AppError::NotFound(format!("No retention policy for table: {}", table_name)))?;

        let start = Instant::now();

        // Step 1: Archive if configured
        let rows_archived = match self.retention_repo.archive_before_cleanup(&policy).await {
            Ok(count) => count,
            Err(e) => {
                let result = RetentionCleanupResult::failure(table_name, format!("Archive failed: {}", e));
                self.log_cleanup_event(&policy, &result).await?;
                return Ok(result);
            }
        };

        // Step 2: Delete expired data
        let rows_deleted = match self.retention_repo.execute_cleanup(&policy).await {
            Ok(count) => count,
            Err(e) => {
                let result = RetentionCleanupResult::failure(table_name, format!("Delete failed: {}", e));
                self.log_cleanup_event(&policy, &result).await?;
                return Ok(result);
            }
        };

        let duration_ms = start.elapsed().as_millis() as u64;

        // Step 3: Record cleanup stats
        self.retention_repo.record_cleanup(table_name, rows_deleted).await?;

        let result = RetentionCleanupResult::success(table_name, rows_deleted, rows_archived, duration_ms);

        // Log individual cleanup event
        self.log_cleanup_event(&policy, &result).await?;

        tracing::info!(
            table = %table_name,
            rows_deleted = rows_deleted,
            rows_archived = rows_archived,
            duration_ms = duration_ms,
            "Retention cleanup completed"
        );

        Ok(result)
    }

    async fn get_policy_status(&self) -> Result<Vec<RetentionPolicy>, AppError> {
        self.retention_repo.list_policies().await
    }

    async fn has_overdue_policies(&self, max_age_hours: i64) -> Result<bool, AppError> {
        let policies = self.retention_repo.list_policies().await?;
        let now = chrono::Utc::now();

        for policy in policies {
            // Skip indefinite retention policies
            if policy.deletion_condition.is_empty()
                || policy.deletion_condition.to_uppercase() == "FALSE"
            {
                continue;
            }

            match policy.last_cleanup_at {
                Some(last) => {
                    let age = now.signed_duration_since(last);
                    if age.num_hours() > max_age_hours {
                        return Ok(true);
                    }
                }
                None => {
                    // Never cleaned up = overdue
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_summary_default() {
        let summary = CleanupSummary::default();
        assert_eq!(summary.policies_processed, 0);
        assert_eq!(summary.total_rows_deleted, 0);
        assert!(!summary.has_errors());
    }

    #[test]
    fn test_cleanup_summary_with_errors() {
        let mut summary = CleanupSummary::default();
        summary.errors.push("test error".to_string());
        assert!(summary.has_errors());
    }
}
