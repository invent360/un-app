//! Settlement service for finance operations
//!
//! Implements settlement logic with:
//! - Allocation management with state transitions
//! - Settlement preparation and execution
//! - Payable balance calculation
//! - Referral liability reservation

use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

use crate::server::repositories::{
    AllocationRepository, AllocationEntry, AllocationState,
    CreateAllocationInput, PoolBalanceSummary, PayableBalances,
    CreditOrderRepository, CreateSettlementInput, Settlement,
    ImmutableAuditRepository, AuditEventBuilder, AuditCategory, AuditEventType, ActorType, AuditOutcome,
    DynOutboxRepository, CreateOutboxEvent,
};
use crate::types::AppError;
use uno_api::services::{AllocationService, AllocationRequest};

/// Dynamic type alias for SettlementService trait object
pub type DynSettlementService = Arc<dyn SettlementServiceTrait + Send + Sync>;

/// Settlement error types
#[derive(Debug, Error)]
pub enum SettlementError {
    #[error("Allocation not found: {0}")]
    AllocationNotFound(Uuid),

    #[error("Settlement not found: {0}")]
    SettlementNotFound(String),

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Duplicate allocation: {0}")]
    DuplicateAllocation(String),

    #[error("Reconciliation failed: {0}")]
    ReconciliationFailed(String),

    #[error("Database error: {0}")]
    Database(String),
}

impl From<SettlementError> for AppError {
    fn from(e: SettlementError) -> Self {
        match e {
            SettlementError::AllocationNotFound(id) => AppError::NotFound(format!("Allocation {}", id)),
            SettlementError::SettlementNotFound(ref_) => AppError::NotFound(format!("Settlement {}", ref_)),
            SettlementError::InvalidStateTransition(msg) => AppError::BadRequest(msg),
            SettlementError::DuplicateAllocation(msg) => AppError::Conflict(msg),
            SettlementError::ReconciliationFailed(msg) => AppError::ValidationError(msg),
            SettlementError::Database(msg) => AppError::Database(msg),
        }
    }
}

/// Input for recording an allocation
#[derive(Debug, Clone)]
pub struct RecordAllocationInput {
    pub license_id: String,
    pub pool_micros: i64,
    pub currency: String,
    pub agreement_version: i32,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
}

/// Input for preparing a settlement
#[derive(Debug, Clone)]
pub struct PrepareSettlementInput {
    pub party_type: String,  // ulo, uno, referral
    pub allocation_ids: Vec<Uuid>,
    pub fee_micros: i64,
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub preparer_id: String,
}

/// Settlement preparation result
#[derive(Debug, Clone)]
pub struct SettlementPreparation {
    pub settlement_ref: String,
    pub total_micros: i64,
    pub fee_micros: i64,
    pub net_micros: i64,
    pub allocation_count: i32,
}

/// Input for executing a settlement
#[derive(Debug, Clone)]
pub struct ExecuteSettlementInput {
    pub settlement_ref: String,
    pub executor_id: String,
    pub provider: String,
    pub provider_ref: String,
}

/// Settlement service trait
#[async_trait]
pub trait SettlementServiceTrait: Send + Sync {
    /// Record a new allocation from an external source
    async fn record_allocation(&self, input: RecordAllocationInput) -> Result<AllocationEntry, SettlementError>;

    /// Get allocation by ID
    async fn get_allocation(&self, id: Uuid) -> Result<Option<AllocationEntry>, SettlementError>;

    /// List allocations for a license
    async fn list_allocations(&self, license_id: &str, limit: i32, offset: i32) -> Result<Vec<AllocationEntry>, SettlementError>;

    /// Get pool balance summary for a license
    async fn get_pool_balance(&self, license_id: &str) -> Result<PoolBalanceSummary, SettlementError>;

    /// Get aggregate payable balances
    async fn get_payable_balances(&self) -> Result<PayableBalances, SettlementError>;

    /// Mark allocations as payable
    async fn mark_payable(&self, ids: &[Uuid], actor_id: &str) -> Result<i64, SettlementError>;

    /// Prepare a settlement (marks allocations as paid when settlement is executed)
    async fn prepare_settlement(&self, input: PrepareSettlementInput) -> Result<SettlementPreparation, SettlementError>;

    /// Approve a prepared settlement
    async fn approve_settlement(&self, settlement_ref: &str, approver_id: &str) -> Result<bool, SettlementError>;

    /// Execute an approved settlement
    async fn execute_settlement(&self, input: ExecuteSettlementInput) -> Result<Settlement, SettlementError>;

    /// List settlements
    async fn list_settlements(&self, limit: i32, offset: i32) -> Result<Vec<Settlement>, SettlementError>;

    /// Get settlement by reference
    async fn get_settlement(&self, settlement_ref: &str) -> Result<Option<Settlement>, SettlementError>;
}

/// Settlement service implementation
pub struct SettlementService {
    allocation_repo: Arc<dyn AllocationRepository + Send + Sync>,
    credit_order_repo: Arc<dyn CreditOrderRepository + Send + Sync>,
    audit_repo: Arc<dyn ImmutableAuditRepository + Send + Sync>,
    outbox_repo: DynOutboxRepository,
    allocation_service: AllocationService,
}

impl SettlementService {
    pub fn new(
        allocation_repo: Arc<dyn AllocationRepository + Send + Sync>,
        credit_order_repo: Arc<dyn CreditOrderRepository + Send + Sync>,
        audit_repo: Arc<dyn ImmutableAuditRepository + Send + Sync>,
        outbox_repo: DynOutboxRepository,
    ) -> Self {
        Self {
            allocation_repo,
            credit_order_repo,
            audit_repo,
            outbox_repo,
            allocation_service: AllocationService::new(),
        }
    }

    /// Generate a unique settlement reference
    fn generate_settlement_ref() -> String {
        format!("STL-{}", Uuid::new_v4().to_string().split('-').next().unwrap_or(""))
    }

    /// Log audit event using builder pattern
    async fn log_audit_event(
        &self,
        event_type: AuditEventType,
        category: AuditCategory,
        actor_type: ActorType,
        actor_id: &str,
        resource_type: &str,
        resource_id: &str,
        action: &str,
        data: serde_json::Value,
    ) {
        let builder = AuditEventBuilder::new(event_type, category)
            .actor(actor_type, actor_id.to_string())
            .resource(resource_type, resource_id)
            .action(action)
            .outcome(AuditOutcome::Success)
            .event_data(data);

        if let Err(e) = self.audit_repo.log_event(builder.build()).await {
            tracing::error!(error = %e, "Failed to record audit event");
        }
    }

    /// Publish outbox event
    async fn publish_event(&self, event_type: &str, payload: serde_json::Value) {
        let event = CreateOutboxEvent {
            event_type: event_type.to_string(),
            event_version: 1,
            aggregate_type: "settlement".to_string(),
            aggregate_id: Uuid::new_v4().to_string(),
            payload,
            metadata: None,
            idempotency_key: Some(Uuid::new_v4().to_string()),
        };

        if let Err(e) = self.outbox_repo.create_event(event).await {
            tracing::error!(error = %e, "Failed to create outbox event");
        }
    }
}

#[async_trait]
impl SettlementServiceTrait for SettlementService {
    async fn record_allocation(&self, input: RecordAllocationInput) -> Result<AllocationEntry, SettlementError> {
        // Check for duplicate if external_ref provided
        if let Some(ref external_ref) = input.external_ref {
            let exists = self.allocation_repo
                .exists_by_external_ref(&input.license_id, external_ref)
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?;

            if exists {
                return Err(SettlementError::DuplicateAllocation(
                    format!("Allocation with external_ref {} already exists for license {}", external_ref, input.license_id)
                ));
            }
        }

        // Use AllocationService to compute the split
        let request = AllocationRequest {
            license_id: input.license_id.clone(),
            pool_micros: input.pool_micros,
            currency: input.currency.clone(),
            agreement_version: input.agreement_version,
            period_start: input.period_start,
            period_end: input.period_end,
            source: input.source.clone(),
            external_ref: input.external_ref.clone(),
        };

        let entry = self.allocation_service
            .create_entry(&request)
            .map_err(|e| SettlementError::ReconciliationFailed(e.to_string()))?;

        // Verify reconciliation
        let sum = entry.ulo_micros + entry.uno_micros + entry.referral_micros;
        if sum != entry.pool_micros {
            return Err(SettlementError::ReconciliationFailed(
                format!("Allocation does not reconcile: {} + {} + {} != {}",
                    entry.ulo_micros, entry.uno_micros, entry.referral_micros, entry.pool_micros)
            ));
        }

        // Persist to database
        let create_input = CreateAllocationInput {
            license_id: entry.license_id.clone(),
            agreement_version: entry.agreement_version,
            pool_micros: entry.pool_micros,
            pool_currency: entry.pool_currency.clone(),
            ulo_micros: entry.ulo_micros,
            uno_micros: entry.uno_micros,
            referral_micros: entry.referral_micros,
            ulo_bps: entry.ulo_bps,
            uno_bps: entry.uno_bps,
            referral_bps: entry.referral_bps,
            remainder_micros: entry.remainder_micros,
            period_start: entry.period_start,
            period_end: entry.period_end,
            source: entry.source.clone(),
            external_ref: entry.external_ref.clone(),
        };

        let id = self.allocation_repo
            .create(create_input)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Log audit
        self.log_audit_event(
            AuditEventType::Create,
            AuditCategory::Finance,
            ActorType::System,
            "system",
            "allocation",
            &id.to_string(),
            "create",
            serde_json::json!({
                "license_id": entry.license_id,
                "pool_micros": entry.pool_micros,
                "ulo_micros": entry.ulo_micros,
                "uno_micros": entry.uno_micros,
                "referral_micros": entry.referral_micros,
            }),
        ).await;

        // Fetch and return the created entry
        self.allocation_repo
            .get(id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?
            .ok_or_else(|| SettlementError::AllocationNotFound(id))
    }

    async fn get_allocation(&self, id: Uuid) -> Result<Option<AllocationEntry>, SettlementError> {
        self.allocation_repo
            .get(id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))
    }

    async fn list_allocations(&self, license_id: &str, limit: i32, offset: i32) -> Result<Vec<AllocationEntry>, SettlementError> {
        self.allocation_repo
            .list_by_license(license_id, limit, offset)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))
    }

    async fn get_pool_balance(&self, license_id: &str) -> Result<PoolBalanceSummary, SettlementError> {
        self.allocation_repo
            .get_pool_balance(license_id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))
    }

    async fn get_payable_balances(&self) -> Result<PayableBalances, SettlementError> {
        self.allocation_repo
            .get_payable_balances()
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))
    }

    async fn mark_payable(&self, ids: &[Uuid], actor_id: &str) -> Result<i64, SettlementError> {
        let count = self.allocation_repo
            .mark_payable(ids, actor_id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Log audit
        self.log_audit_event(
            AuditEventType::Update,
            AuditCategory::Finance,
            ActorType::User,
            actor_id,
            "allocation",
            &format!("{} allocations", ids.len()),
            "mark_payable",
            serde_json::json!({
                "action": "mark_payable",
                "count": count,
                "ids": ids,
            }),
        ).await;

        Ok(count)
    }

    async fn prepare_settlement(&self, input: PrepareSettlementInput) -> Result<SettlementPreparation, SettlementError> {
        // Calculate total from allocations
        let mut total_micros: i64 = 0;

        for id in &input.allocation_ids {
            let alloc = self.allocation_repo
                .get(*id)
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?
                .ok_or_else(|| SettlementError::AllocationNotFound(*id))?;

            // Sum the appropriate party's share
            let party_amount = match input.party_type.as_str() {
                "ulo" => alloc.ulo_micros,
                "uno" => alloc.uno_micros,
                "referral" => alloc.referral_micros,
                _ => return Err(SettlementError::InvalidStateTransition(
                    format!("Invalid party type: {}", input.party_type)
                )),
            };

            total_micros += party_amount;
        }

        let net_micros = total_micros - input.fee_micros;
        let settlement_ref = Self::generate_settlement_ref();

        // Create settlement record
        let create_input = CreateSettlementInput {
            settlement_ref: settlement_ref.clone(),
            party_type: input.party_type.clone(),
            total_micros,
            fee_micros: input.fee_micros,
            currency: input.currency.clone(),
            period_start: input.period_start,
            period_end: input.period_end,
            allocation_ids: input.allocation_ids.clone(),
            prepared_by: input.preparer_id.clone(),
        };

        self.credit_order_repo
            .create_settlement(create_input)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Log audit
        self.log_audit_event(
            AuditEventType::Create,
            AuditCategory::Finance,
            ActorType::User,
            &input.preparer_id,
            "settlement",
            &settlement_ref,
            "prepare",
            serde_json::json!({
                "party_type": input.party_type,
                "total_micros": total_micros,
                "fee_micros": input.fee_micros,
                "net_micros": net_micros,
                "allocation_count": input.allocation_ids.len(),
            }),
        ).await;

        Ok(SettlementPreparation {
            settlement_ref,
            total_micros,
            fee_micros: input.fee_micros,
            net_micros,
            allocation_count: input.allocation_ids.len() as i32,
        })
    }

    async fn approve_settlement(&self, settlement_ref: &str, approver_id: &str) -> Result<bool, SettlementError> {
        let settlement = self.credit_order_repo
            .get_settlement_by_ref(settlement_ref)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?
            .ok_or_else(|| SettlementError::SettlementNotFound(settlement_ref.to_string()))?;

        let success = self.credit_order_repo
            .approve_settlement(settlement.id, approver_id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        if success {
            self.log_audit_event(
                AuditEventType::Update,
                AuditCategory::Finance,
                ActorType::User,
                approver_id,
                "settlement",
                settlement_ref,
                "approve",
                serde_json::json!({
                    "action": "approved",
                }),
            ).await;
        }

        Ok(success)
    }

    async fn execute_settlement(&self, input: ExecuteSettlementInput) -> Result<Settlement, SettlementError> {
        let settlement = self.credit_order_repo
            .get_settlement_by_ref(&input.settlement_ref)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?
            .ok_or_else(|| SettlementError::SettlementNotFound(input.settlement_ref.clone()))?;

        // Execute settlement
        let success = self.credit_order_repo
            .execute_settlement(settlement.id, &input.executor_id, &input.provider, &input.provider_ref)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        if !success {
            return Err(SettlementError::InvalidStateTransition(
                "Settlement must be in approved state to execute".to_string()
            ));
        }

        // Mark all allocations as paid
        self.allocation_repo
            .mark_paid(&settlement.allocation_ids, &input.executor_id, &input.settlement_ref)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Log audit
        self.log_audit_event(
            AuditEventType::Update,
            AuditCategory::Finance,
            ActorType::User,
            &input.executor_id,
            "settlement",
            &input.settlement_ref,
            "execute",
            serde_json::json!({
                "action": "executed",
                "provider": input.provider,
                "provider_ref": input.provider_ref,
            }),
        ).await;

        // Publish event
        self.publish_event("settlement.executed", serde_json::json!({
            "settlement_ref": input.settlement_ref,
            "total_micros": settlement.total_micros,
            "party_type": settlement.party_type,
        })).await;

        // Return updated settlement
        self.credit_order_repo
            .get_settlement(settlement.id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?
            .ok_or_else(|| SettlementError::SettlementNotFound(input.settlement_ref))
    }

    async fn list_settlements(&self, limit: i32, offset: i32) -> Result<Vec<Settlement>, SettlementError> {
        self.credit_order_repo
            .list_settlements(limit, offset)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))
    }

    async fn get_settlement(&self, settlement_ref: &str) -> Result<Option<Settlement>, SettlementError> {
        self.credit_order_repo
            .get_settlement_by_ref(settlement_ref)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))
    }
}
