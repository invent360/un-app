//! Database access layer with trait-based repositories

mod license_repository;
mod stats_repository;
mod faq_repository;
mod content_repository;
mod referral_repository;
mod claim_repository;
mod review_repository;
mod audit_repository;
mod rbac_repository;
mod schema_repository;
mod content_item_repository;
mod testimonial_repository;
mod session_repository;
mod launch_gate_repository;
mod consent_repository;
mod outbox_repository;
mod service_identity_repository;
mod sync_checkpoint_repository;
mod import_repository;
mod publication_repository;
mod eligibility_repository;
mod nonce_repository;
mod allocation_repository;
mod credit_order_repository;
mod media_asset_repository;
mod media_backup_repository;
mod support_repository;
mod cohort_repository;
mod market_repository;
mod exit_repository;
mod operator_metrics_repository;
mod forecast_repository;
mod webhook_repository;
mod communication_repository;

// Re-export implementations
pub use license_repository::PostgresLicenseRepository;
pub use stats_repository::StatsRepositoryImpl;
pub use faq_repository::FaqRepositoryImpl;
pub use content_repository::ContentRepositoryImpl;
pub use referral_repository::ReferralRepositoryImpl;
pub use claim_repository::ClaimRepositoryImpl;
pub use review_repository::ReviewRepositoryImpl;
pub use audit_repository::{AuditRepositoryImpl, ImmutableAuditRepositoryImpl};
pub use rbac_repository::RbacRepositoryImpl;
pub use schema_repository::SchemaRepositoryImpl;
pub use content_item_repository::ContentItemRepositoryImpl;
pub use testimonial_repository::TestimonialRepositoryImpl;
pub use session_repository::SessionRepositoryImpl;
pub use launch_gate_repository::LaunchGateRepositoryImpl;
pub use consent_repository::ConsentRepositoryImpl;
pub use outbox_repository::OutboxRepositoryImpl;
pub use service_identity_repository::ServiceIdentityRepositoryImpl;
pub use sync_checkpoint_repository::SyncCheckpointRepositoryImpl;
pub use import_repository::ImportRepositoryImpl;
pub use publication_repository::PublicationRepositoryImpl;
pub use eligibility_repository::EligibilityRepositoryImpl;
pub use nonce_repository::NonceRepositoryImpl;

// Re-export dynamic type aliases
pub use license_repository::DynLicenseRepository;
pub use stats_repository::DynStatsRepository;
pub use faq_repository::DynFaqRepository;
pub use content_repository::DynContentRepository;
pub use referral_repository::DynReferralRepository;
pub use claim_repository::DynClaimRepository;
pub use review_repository::DynReviewRepository;
pub use audit_repository::{DynAuditRepository, DynImmutableAuditRepository};
pub use rbac_repository::DynRbacRepository;
pub use schema_repository::DynSchemaRepository;
pub use content_item_repository::DynContentItemRepository;
pub use testimonial_repository::DynTestimonialRepository;
pub use session_repository::DynSessionRepository;
pub use launch_gate_repository::DynLaunchGateRepository;
pub use consent_repository::DynConsentRepository;
pub use outbox_repository::DynOutboxRepository;
pub use service_identity_repository::DynServiceIdentityRepository;
pub use sync_checkpoint_repository::DynSyncCheckpointRepository;
pub use import_repository::DynImportRepository;
pub use publication_repository::DynPublicationRepository;
pub use eligibility_repository::DynEligibilityRepository;
pub use nonce_repository::DynNonceRepository;

// Re-export repository traits from stats and faq (license uses uno-api trait)
pub use stats_repository::StatsRepository;
pub use faq_repository::FaqRepository;
pub use content_repository::ContentRepository;
pub use referral_repository::{ReferralRepository, ReferralInput};
pub use claim_repository::{ClaimRepository, ReservationResult, ClaimResult, RESERVATION_EXPIRY_SECS};
pub use review_repository::ReviewRepository;
pub use audit_repository::AuditRepository;
// New immutable audit repository types for audit_log table (migration 00020)
pub use audit_repository::{
    ImmutableAuditRepository, ImmutableAuditLogEntry,
    AuditEvent, AuditEventBuilder,
    AuditCategory, AuditEventType, ActorType, AuditOutcome,
};
pub use rbac_repository::RbacRepository;
pub use schema_repository::SchemaRepository;
pub use content_item_repository::ContentItemRepository;
pub use testimonial_repository::{TestimonialRepository, Testimonial};
pub use session_repository::{
    SessionRepository, UserIdentity, Session, SessionMetadata,
    BlacklistEntry, CleanupResult, hash_token
};
pub use launch_gate_repository::{
    LaunchGateRepository, LaunchGate, LaunchGateHistory, GateName
};
pub use consent_repository::{
    ConsentRepository, ConsentVersion, UserConsent, DataAccessRequest,
    DataRetentionPolicy, RecordConsentInput, CreateDataRequestInput,
    UserConsentStatus, ConsentSummary
};
pub use outbox_repository::{
    OutboxRepository, OutboxEvent, OutboxStatus, CreateOutboxEvent,
    InboxEvent, InboxStatus, WorkerLease
};
pub use service_identity_repository::{
    ServiceIdentityRepository, ServiceIdentity, ServiceType, RateLimitTier,
    CreateServiceIdentityInput, RotateKeyInput, ServiceIdentityAudit,
    hash_key, key_hint,
};
pub use sync_checkpoint_repository::{
    SyncCheckpointRepository, SyncCheckpoint, SaveCheckpointInput,
    ReconcileCheckpointInput, RecordSyncErrorInput, CursorValue,
};
pub use import_repository::{
    ImportRepository, ImportBatch, ImportRow, ImportSummary, ImportRowError,
    ImportMode, ImportStatus, RowStatus, RowAction,
    CreateImportBatchInput, CreateImportRowInput,
    UpdateRowValidationInput, UpdateRowActionInput,
};
pub use publication_repository::{
    PublicationRepository, PublicationBatch, PublicationItem, PublicationSummary, FailedItem,
    PublicationOperation, PublicationStatus, ItemStatus, ItemResult, LicensePublicationStatus,
    CreatePublicationBatchInput, AcknowledgeItemsInput,
};
pub use eligibility_repository::{
    EligibilityRepository, EligibilityRuleset, LicenseEligibility,
    EligibilityContext, EligibilityResult, EligibilityFailure,
    DeviceType, CheckType, CreateRulesetInput,
};
pub use nonce_repository::NonceRepository;
pub use allocation_repository::{
    AllocationRepository, AllocationRepositoryImpl, DynAllocationRepository,
    AllocationEntry, AllocationState, CreateAllocationInput,
    PoolBalanceSummary, PayableBalances,
};
pub use credit_order_repository::{
    CreditOrderRepository, CreditOrderRepositoryImpl, DynCreditOrderRepository,
    CreditOrder, CreditOrderState, PayerType,
    CreateCreditOrderInput, ApproveCreditOrderInput, ConfirmCreditOrderInput,
    Settlement, CreateSettlementInput,
};
pub use media_asset_repository::{
    MediaAssetRepository, MediaAssetRepositoryImpl, DynMediaAssetRepository,
    MediaAsset, MediaAssetGrant, MediaQuota, MediaAssetVersion,
    AssetState, QuotaCheckResult,
    CreateAssetInput, UpdateStateInput, CreateGrantInput, SetQuotaInput,
};
pub use media_backup_repository::{
    MediaBackupRepository, MediaBackupRepositoryImpl, DynMediaBackupRepository,
    MediaBackup, MediaBackupEntry, MediaRestore, StorageHealthMetric,
    BackupStatus, BackupType, RestoreStatus,
    CreateBackupInput, CreateBackupEntryInput, CreateRestoreInput, RecordStorageMetricsInput,
};
pub use support_repository::{
    SupportRepository, SupportRepositoryImpl, DynSupportRepository,
    SupportTicket, TicketMessage, TicketHistory, QueueAssignment, CannedResponse,
    TicketStatus, TicketPriority, TicketCategory,
    CreateTicketInput, AddMessageInput, TicketFilters, QueueStats,
};
pub use cohort_repository::{
    CohortRepository, CohortRepositoryImpl, DynCohortRepository,
    ParticipantCohort, CohortDailyActivity, CohortNotification, CohortAnalytics,
    CreateCohortInput, RecordActivityInput, ScheduleNotificationInput,
    D7Progress, D30Progress, CohortStats,
};
pub use market_repository::{
    MarketRepository, MarketRepositoryImpl, DynMarketRepository,
    MarketStatus, MarketQuota, QuotaConsumption, CampaignSource, CampaignAttribution,
    UpsertMarketStatusInput, CreateQuotaInput, CreateCampaignInput, RecordAttributionInput,
    MarketReadiness, QuotaStatus,
};
pub use exit_repository::{
    ExitRepository, ExitRepositoryImpl, DynExitRepository,
    ParticipantExit, ExitFeedback, ExitAuditLog, WaitlistEntry,
    ExitType, ExitStatus, PayoutStatus,
    InitiateExitInput, SubmitFeedbackInput, AddToWaitlistInput,
    ExitBalance, PayoutResult,
};

// Phase 8: Operator Tools, Forecasting and Optional Adapters
pub use operator_metrics_repository::{
    OperatorMetricsRepository, OperatorMetricsRepositoryImpl, DynOperatorMetricsRepository,
    OperatorMetrics, OperatorException, OperatorMargin, ExceptionSeverity,
    RecordMetricsInput, RecordExceptionInput, RecordMarginInput,
    InventorySummary, CohortSummary, FinancialSummary, SupportSummary, SyncStatus,
};
pub use forecast_repository::{
    ForecastRepository, ForecastRepositoryImpl, DynForecastRepository,
    ForecastScenario, ForecastTask, ForecastResult, ForecastGoldenFixture, ForecastScenarioSnapshot,
    ForecastStatus, CreateScenarioInput, UpdateScenarioInput, AddTaskInput, UpdateTaskInput,
    CreateGoldenFixtureInput, ScenarioExport, ValidationResult, ValidationDifference,
};
pub use webhook_repository::{
    WebhookRepository, WebhookRepositoryImpl, DynWebhookRepository,
    WebhookEndpoint, WebhookDelivery, InboundWebhook, WebhookSourceConfig,
    WebhookAuthType, WebhookDeliveryStatus, InboundWebhookStatus,
    CreateEndpointInput, UpdateEndpointInput, CreateSourceConfigInput, DeliveryResultInput,
    DeliveryStats, ProcessingStats,
};
pub use communication_repository::{
    CommunicationRepository, CommunicationRepositoryImpl, DynCommunicationRepository,
    CommunicationPreferences, CommunicationSuppression, MessageTemplate, ScheduledMessage, DeviceToken,
    CommChannel, MessageCategory, SuppressionType, TemplateStatus, ScheduledMessageStatus,
    UpdatePreferencesInput, AddSuppressionInput, CreateTemplateInput, UpdateTemplateInput,
    ScheduleMessageInput, RegisterDeviceInput, CanReceiveResult, SendStats,
};
