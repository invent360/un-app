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
    // R4-04: Settlement items for per-party settlement
    SettlementItemRepository, CreateSettlementItemInput,
};
use std::collections::HashSet;
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
    /// R3-08: Provider identifier for deduplication (e.g., "unetwork", "marketplace")
    pub provider_id: Option<String>,
    /// R3-08: Unique event ID from provider for duplicate prevention
    pub reward_event_id: Option<String>,
    /// R5-07: Referral agent ID (if license was referred).
    /// When None, allocation uses allocate_without_referral() and share goes to reserve.
    pub referral_agent_id: Option<Uuid>,
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
/// R5-08: Settlement item repository is REQUIRED in production for per-party settlement
pub struct SettlementService {
    allocation_repo: Arc<dyn AllocationRepository + Send + Sync>,
    credit_order_repo: Arc<dyn CreditOrderRepository + Send + Sync>,
    audit_repo: Arc<dyn ImmutableAuditRepository + Send + Sync>,
    outbox_repo: DynOutboxRepository,
    // R5-08: Repository is required, not optional - enables per-party settlement tracking
    settlement_item_repo: Arc<dyn crate::server::repositories::SettlementItemRepository + Send + Sync>,
    allocation_service: AllocationService,
}

impl SettlementService {
    // R5-08: Single constructor requires settlement item repository
    // The old `new()` constructor without repository has been removed
    pub fn new(
        allocation_repo: Arc<dyn AllocationRepository + Send + Sync>,
        credit_order_repo: Arc<dyn CreditOrderRepository + Send + Sync>,
        audit_repo: Arc<dyn ImmutableAuditRepository + Send + Sync>,
        outbox_repo: DynOutboxRepository,
        settlement_item_repo: Arc<dyn crate::server::repositories::SettlementItemRepository + Send + Sync>,
    ) -> Self {
        Self {
            allocation_repo,
            credit_order_repo,
            audit_repo,
            outbox_repo,
            settlement_item_repo,
            allocation_service: AllocationService::new(),
        }
    }

    // R5-08: Deprecated alias - use new() instead
    #[deprecated(note = "Use new() - settlement_item_repo is now required")]
    pub fn with_settlement_items(
        allocation_repo: Arc<dyn AllocationRepository + Send + Sync>,
        credit_order_repo: Arc<dyn CreditOrderRepository + Send + Sync>,
        audit_repo: Arc<dyn ImmutableAuditRepository + Send + Sync>,
        outbox_repo: DynOutboxRepository,
        settlement_item_repo: Arc<dyn crate::server::repositories::SettlementItemRepository + Send + Sync>,
    ) -> Self {
        Self::new(allocation_repo, credit_order_repo, audit_repo, outbox_repo, settlement_item_repo)
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

    /// R5-08: Check if an allocation has all parties settled
    /// Returns true only when ULO, UNO, referral (if applicable), and reserve are all confirmed
    async fn is_allocation_fully_settled(&self, allocation_id: Uuid) -> Result<bool, SettlementError> {
        // Get the allocation to check which parties have amounts > 0
        let alloc = self.allocation_repo
            .get(allocation_id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?
            .ok_or_else(|| SettlementError::AllocationNotFound(allocation_id))?;

        // R5-08: Get all settlement items for this allocation (repository is now required)
        let items = self.settlement_item_repo
            .list_by_allocation(allocation_id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Build a map of party_type -> is_confirmed
        let mut party_confirmed: std::collections::HashMap<&str, bool> = std::collections::HashMap::new();
        for item in &items {
            if item.item_state == "confirmed" {
                party_confirmed.insert(item.party_type.as_str(), true);
            }
        }

        // Check each party that has a non-zero amount
        let mut all_settled = true;

        if alloc.ulo_micros > 0 && !party_confirmed.get("ulo").copied().unwrap_or(false) {
            all_settled = false;
        }
        if alloc.uno_micros > 0 && !party_confirmed.get("uno").copied().unwrap_or(false) {
            all_settled = false;
        }
        if alloc.referral_micros > 0 && !party_confirmed.get("referral").copied().unwrap_or(false) {
            all_settled = false;
        }
        if alloc.reserve_micros > 0 && !party_confirmed.get("reserve").copied().unwrap_or(false) {
            all_settled = false;
        }

        Ok(all_settled)
    }
}

#[async_trait]
impl SettlementServiceTrait for SettlementService {
    async fn record_allocation(&self, input: RecordAllocationInput) -> Result<AllocationEntry, SettlementError> {
        // R3-08: Check for duplicate by provider + event_id first (authoritative deduplication)
        if let (Some(ref provider_id), Some(ref reward_event_id)) = (&input.provider_id, &input.reward_event_id) {
            let exists = self.allocation_repo
                .exists_by_provider_event(provider_id, reward_event_id)
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?;

            if exists {
                return Err(SettlementError::DuplicateAllocation(
                    format!("Allocation with provider_id {} and reward_event_id {} already exists",
                        provider_id, reward_event_id)
                ));
            }
        }

        // Legacy check: duplicate by external_ref + license_id
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
        // R5-07: Include referral_agent_id for proper reserve allocation
        let request = AllocationRequest {
            license_id: input.license_id.clone(),
            pool_micros: input.pool_micros,
            currency: input.currency.clone(),
            agreement_version: input.agreement_version,
            period_start: input.period_start,
            period_end: input.period_end,
            source: input.source.clone(),
            external_ref: input.external_ref.clone(),
            provider_id: input.provider_id.clone(),
            reward_event_id: input.reward_event_id.clone(),
            referral_agent_id: input.referral_agent_id,
        };

        let entry = self.allocation_service
            .create_entry(&request)
            .map_err(|e| SettlementError::ReconciliationFailed(e.to_string()))?;

        // Verify reconciliation (R3-08: includes reserve_micros)
        let sum = entry.ulo_micros + entry.uno_micros + entry.referral_micros + entry.reserve_micros;
        if sum != entry.pool_micros {
            return Err(SettlementError::ReconciliationFailed(
                format!("Allocation does not reconcile: {} + {} + {} + {} != {}",
                    entry.ulo_micros, entry.uno_micros, entry.referral_micros,
                    entry.reserve_micros, entry.pool_micros)
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
            reserve_micros: entry.reserve_micros,
            ulo_bps: entry.ulo_bps,
            uno_bps: entry.uno_bps,
            referral_bps: entry.referral_bps,
            remainder_micros: entry.remainder_micros,
            period_start: entry.period_start,
            period_end: entry.period_end,
            source: entry.source.clone(),
            external_ref: entry.external_ref.clone(),
            provider_id: entry.provider_id.clone(),
            reward_event_id: entry.reward_event_id.clone(),
            referral_agent_id: entry.referral_agent_id,
            // R5-07: Reward source tracking (defaults for automatic allocations)
            reward_source: Some("automatic".to_string()),
            reward_batch_id: None,
            is_batch_member: false,
            agreement_snapshot: None,
            ulo_recipient_id: None,
            uno_recipient_id: None,
            reserve_recipient_id: None,
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
        // R4-04: Validate nonempty distinct IDs
        if input.allocation_ids.is_empty() {
            return Err(SettlementError::ReconciliationFailed(
                "Settlement requires at least one allocation".to_string()
            ));
        }

        let unique_ids: HashSet<_> = input.allocation_ids.iter().collect();
        if unique_ids.len() != input.allocation_ids.len() {
            return Err(SettlementError::DuplicateAllocation(
                "Settlement contains duplicate allocation IDs".to_string()
            ));
        }

        // R4-04: Validate party type
        let valid_parties = ["ulo", "uno", "referral", "reserve"];
        if !valid_parties.contains(&input.party_type.as_str()) {
            return Err(SettlementError::InvalidStateTransition(
                format!("Invalid party type: {}", input.party_type)
            ));
        }

        // R5-08: Validate fee bounds (0 <= fee <= total will be checked after total is computed)
        if input.fee_micros < 0 {
            return Err(SettlementError::ReconciliationFailed(
                format!("Fee must be non-negative, got: {}", input.fee_micros)
            ));
        }

        // Calculate total from allocations with validation
        let mut total_micros: i64 = 0;
        let mut expected_currency: Option<String> = None;
        let mut allocation_items: Vec<(Uuid, i64, String)> = Vec::new(); // (id, amount, recipient_id)

        for id in &input.allocation_ids {
            let alloc = self.allocation_repo
                .get(*id)
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?
                .ok_or_else(|| SettlementError::AllocationNotFound(*id))?;

            // R4-04: Validate payable state
            if alloc.state.as_deref() != Some("payable") {
                return Err(SettlementError::InvalidStateTransition(
                    format!("Allocation {} is not in payable state", id)
                ));
            }

            // R4-04: Validate currency consistency
            if let Some(ref currency) = expected_currency {
                if &alloc.pool_currency != currency {
                    return Err(SettlementError::ReconciliationFailed(
                        format!("Currency mismatch: expected {}, found {} in allocation {}",
                            currency, alloc.pool_currency, id)
                    ));
                }
            } else {
                expected_currency = Some(alloc.pool_currency.clone());
            }

            // R5-08: Check if this allocation+party is already in a pending settlement
            let is_pending = self.settlement_item_repo
                .is_pending(*id, &input.party_type)
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?;

            if is_pending {
                return Err(SettlementError::DuplicateAllocation(
                    format!("Allocation {} party {} is already in a pending settlement", id, input.party_type)
                ));
            }

            // R5-08: Sum the appropriate party's share with recipient validation
            // Validate that recipient is an actual payee, not just license_id
            let (party_amount, recipient_id) = match input.party_type.as_str() {
                "ulo" => {
                    // R5-08: ULO recipient should resolve to actual user/participant
                    // For now, use ulo_recipient_id if available, fall back to license_id
                    let recipient = alloc.ulo_recipient_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| alloc.license_id.clone());
                    (alloc.ulo_micros, recipient)
                },
                "uno" => {
                    // UNO recipient is the treasury (system recipient)
                    let recipient = alloc.uno_recipient_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "uno_treasury".to_string());
                    (alloc.uno_micros, recipient)
                },
                "referral" => {
                    // R5-08: Referral recipient must be the actual agent
                    let recipient = alloc.referral_agent_id
                        .map(|id| id.to_string())
                        .unwrap_or_default();

                    // Reject if referral amount > 0 but no agent assigned
                    if alloc.referral_micros > 0 && recipient.is_empty() {
                        return Err(SettlementError::ReconciliationFailed(
                            format!("Allocation {} has referral_micros {} but no referral_agent_id",
                                id, alloc.referral_micros)
                        ));
                    }
                    (alloc.referral_micros, recipient)
                },
                "reserve" => {
                    // Reserve recipient is the reserve fund (system recipient)
                    let recipient = alloc.reserve_recipient_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "reserve_fund".to_string());
                    (alloc.reserve_micros, recipient)
                },
                _ => return Err(SettlementError::InvalidStateTransition(
                    format!("Invalid party type: {}", input.party_type)
                )),
            };

            // R4-04: Checked integer arithmetic
            total_micros = total_micros.checked_add(party_amount)
                .ok_or_else(|| SettlementError::ReconciliationFailed(
                    "Settlement total overflow".to_string()
                ))?;

            allocation_items.push((*id, party_amount, recipient_id));
        }

        // R4-04: Validate currency matches input
        if let Some(ref actual_currency) = expected_currency {
            if actual_currency != &input.currency {
                return Err(SettlementError::ReconciliationFailed(
                    format!("Settlement currency {} doesn't match allocation currency {}",
                        input.currency, actual_currency)
                ));
            }
        }

        // R5-08: Enforce 0 <= fee <= total explicitly
        if input.fee_micros > total_micros {
            return Err(SettlementError::ReconciliationFailed(
                format!("Fee {} exceeds total {}", input.fee_micros, total_micros)
            ));
        }

        // Net cannot be negative after fee validation above
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

        let settlement_id = self.credit_order_repo
            .create_settlement(create_input)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // R5-08: Create settlement items for each allocation (repository is now required)
        let recipient_type = match input.party_type.as_str() {
            "ulo" => "license_owner",
            "uno" => "uno_treasury",
            "referral" => "referral_agent",
            "reserve" => "reserve_fund",
            _ => "unknown",
        };

        for (alloc_id, amount, recipient_id) in &allocation_items {
            let item_input = CreateSettlementItemInput {
                settlement_id,
                allocation_id: *alloc_id,
                party_type: input.party_type.clone(),
                recipient_id: if recipient_id.is_empty() { None } else { Some(recipient_id.clone()) },
                recipient_type: Some(recipient_type.to_string()),
                amount_micros: *amount,
                currency: input.currency.clone(),
            };

            self.settlement_item_repo.create(item_input)
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?;

            // Update per-party settlement state
            self.settlement_item_repo.update_allocation_settlement_state(*alloc_id, &input.party_type, "submitted")
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?;
        }

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

        // R5-08: Check for provider_ref idempotency (prevent duplicate confirmations)
        let existing = self.settlement_item_repo
            .list_by_settlement(settlement.id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Check if any items are already confirmed with a provider ref
        for item in &existing {
            if item.item_state == "confirmed" {
                tracing::warn!(
                    settlement_id = %settlement.id,
                    item_id = %item.id,
                    "Settlement items already confirmed - idempotent return"
                );
                // Idempotent: already executed, return current state
                return self.credit_order_repo
                    .get_settlement(settlement.id)
                    .await
                    .map_err(|e| SettlementError::Database(e.to_string()))?
                    .ok_or_else(|| SettlementError::SettlementNotFound(input.settlement_ref));
            }
        }

        // Execute settlement record
        let success = self.credit_order_repo
            .execute_settlement(settlement.id, &input.executor_id, &input.provider, &input.provider_ref)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        if !success {
            return Err(SettlementError::InvalidStateTransition(
                "Settlement must be in approved state to execute".to_string()
            ));
        }

        // R5-08: Mark settlement items as confirmed and update per-party states
        // DO NOT mark whole allocation as paid - only update party-specific state
        // Get all settlement items
        let items = self.settlement_item_repo
            .list_by_settlement(settlement.id)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Mark items as confirmed
        let item_ids: Vec<Uuid> = items.iter().map(|i| i.id).collect();
        self.settlement_item_repo
            .mark_confirmed(&item_ids)
            .await
            .map_err(|e| SettlementError::Database(e.to_string()))?;

        // Update per-party settlement state to "confirmed" for each allocation
        for item in &items {
            self.settlement_item_repo
                .update_allocation_settlement_state(item.allocation_id, &settlement.party_type, "confirmed")
                .await
                .map_err(|e| SettlementError::Database(e.to_string()))?;

            // R5-08: Check if all parties are now settled for this allocation
            // Only mark whole allocation as "paid" when ALL parties are confirmed
            if self.is_allocation_fully_settled(item.allocation_id).await? {
                self.allocation_repo
                    .mark_paid(&[item.allocation_id], &input.executor_id, &input.settlement_ref)
                    .await
                    .map_err(|e| SettlementError::Database(e.to_string()))?;
            }
        }

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
                "party_type": settlement.party_type,
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
