//! Business logic layer with service implementations
//!
//! This module provides service traits and implementations for the application's
//! business logic. Services are the primary way to interact with business operations.
//!
//! # Architecture
//!
//! ```text
//! Handler -> Service -> Repository (uno-api trait) -> PostgresImpl -> DB
//! ```

pub mod chatbot_service;
pub mod traits;
mod license_service;
mod faq_service;
mod stats_service;
mod content_service;
mod audit_service;
mod rbac_service;
mod schema_service;
mod content_item_service;
mod launch_gate_service;
mod session_service;
mod consent_service;
mod job_service;
mod worker_runner;
mod service_identity_service;
mod evidence_service;
mod reservation_service;
mod ownership_service;
mod agent_service;
mod lifecycle_service;
mod journey_service;
mod outbox_publisher;
mod inbox_processor;
mod settlement_service;
mod media_asset_service;
mod media_backup_service;
mod locale_review_service;
mod cohort_notification_service;
mod forecast_service;
mod webhook_service;
mod communication_service;
mod pilot_service;
mod retention_service;

// Re-export service implementations
pub use chatbot_service::ChatbotService;
pub use license_service::{LicenseService, SplitTypeAvailability};
pub use faq_service::FaqServiceImpl;
pub use stats_service::StatsServiceImpl;
pub use content_service::ContentServiceImpl;
pub use audit_service::AuditServiceImpl;
pub use rbac_service::RbacServiceImpl;
pub use schema_service::SchemaServiceImpl;
pub use content_item_service::ContentItemServiceImpl;
pub use launch_gate_service::LaunchGateService;
pub use session_service::SessionService;
pub use consent_service::ConsentService;
pub use job_service::{
    JobService, JobConfig, JobPriority, JobStatus, JobOutcome,
    CreateJobInput, Job, JobResult,
};
pub use worker_runner::{
    WorkerRunner, WorkerConfig, WorkerStats,
    run_lease_reclaimer, run_event_cleanup, run_stale_event_recovery, run_retention_cleanup,
    run_outbox_publisher, run_outbox_stale_recovery,
};
pub use outbox_publisher::OutboxPublisherConfig;
pub use service_identity_service::{ServiceIdentityService, AuthenticatedService};
pub use evidence_service::{
    EvidenceService, EvidenceProvider, DynEvidenceProvider,
    EvidenceRecord, EvidenceType, EvidenceStatus,
    RecordEvidenceInput, ManualVerificationInput,
    NullEvidenceProvider,
};
pub use reservation_service::{
    ReservationService, ReservationServiceImpl, DynReservationService,
    ReservationRequest, ConfirmRequest, ExtendedReservationResult, ReservationError,
    MAX_OCCUPIED_LICENSES,
};
pub use ownership_service::{
    OwnershipService, OwnershipServiceImpl, DynOwnershipService,
    LicenseOwnership, EstablishOwnershipInput, GateValidationResult, VerificationCheckResult,
    OwnerType, AcquisitionType, VerificationMethod, VerificationContext,
};
pub use agent_service::{
    AgentService, AgentServiceImpl, DynAgentService,
    Agent, AgentStatus, AgentStatusHistory, AgentSummary,
    CreateAgentInput, ApproveAgentInput, RejectAgentInput, SuspendAgentInput, LiftSuspensionInput,
};
pub use lifecycle_service::{
    LifecycleService, LifecycleServiceImpl, DynLifecycleService,
    LifecycleEvent, ActorType as LifecycleActorType, LifecycleLogEntry, ExposureMetrics,
    CancelLicenseInput, ReleaseLicenseInput, ReactivateLicenseInput,
    ExpiryNotificationType, LicensePendingExpiry,
    // R5-05: Release workflow types
    ReleaseStatus, RequestReleaseInput, RequestReleaseResult, ConfirmUpstreamReleaseInput,
};
pub use journey_service::{
    JourneyService, JourneyServiceImpl, DynJourneyService,
    JourneyEntry, StageMetrics,
};
pub use outbox_publisher::{
    OutboxPublisher, PublishStats, WebhookPayload,
    EventPublisher, DynEventPublisher, WebhookPublisher, NullPublisher,
};
pub use inbox_processor::{
    InboxProcessor, InboxOutcome, InboxContext,
    InboxHandler, DynInboxHandler, LoggingHandler,
};
pub use settlement_service::{
    SettlementService, SettlementServiceTrait, DynSettlementService, SettlementError,
    RecordAllocationInput, PrepareSettlementInput, ExecuteSettlementInput, SettlementPreparation,
};
pub use media_asset_service::{
    MediaAssetService, MediaAssetServiceImpl, DynMediaAssetService,
    TrackedUploadResult, UploadRequest, DiskThresholdStatus, ThresholdLevel,
    ReconcileResult, AssetVerification,
};
pub use media_backup_service::{
    MediaBackupService, MediaBackupServiceImpl, DynMediaBackupService,
    BackupResult, RestoreResult, BackupRequest, RestoreRequest, BackupVerification,
};
pub use locale_review_service::{
    LocaleReviewService, LocaleReviewServiceImpl, DynLocaleReviewService,
    LocaleReview, LocaleReviewStatus,
    SubmitLocaleReviewInput, ApproveLocaleInput, RequestChangesInput,
};
pub use cohort_notification_service::{
    CohortNotificationService, CohortNotificationConfig,
    CohortNotificationType, NotificationChannel,
    CohortNotificationPayload, MilestoneCheckPayload, ScheduleNotificationsPayload,
    ProcessingStats, MilestoneCheckStats,
    JOB_TYPE_COHORT_NOTIFICATION, JOB_TYPE_COHORT_MILESTONE_CHECK, JOB_TYPE_COHORT_SCHEDULE_NOTIFICATIONS,
    process_pending_notifications, check_pending_milestones, create_cohort_job_handler,
};

// Phase 8: Operator Tools, Forecasting and Optional Adapters
// R5-12: ForecastEngine removed - all forecasts use canonical engine
pub use forecast_service::{
    ForecastService, ForecastServiceImpl, DynForecastService,
    ForecastEngineConfig, AgreementShares, WeeklyProjection,
};
pub use webhook_service::{
    WebhookService, WebhookServiceImpl, DynWebhookService,
    WebhookHttpClient, DefaultHttpClient, HttpResponse, HttpError,
    VerificationResult as WebhookVerificationResult,
};
pub use communication_service::{
    CommunicationService, CommunicationServiceImpl, DynCommunicationService,
    EmailProvider, PushProvider, NullEmailProvider, NullPushProvider,
    EmailSendResult, PushSendResult, SendMessageResult, RenderedMessage,
};

// Phase 9: Pilot management
pub use pilot_service::{
    PilotService, PilotServiceImpl, DynPilotService,
};

// R3-17: Data retention policy enforcement
pub use retention_service::{
    RetentionService, RetentionServiceImpl, DynRetentionService,
    CleanupSummary,
};

// Re-export service traits and types
pub use traits::{
    // Stats service
    StatsService,
    DynStatsService,
    StatsPeriod,
    LicenseStats,
    ClaimStats,
    // FAQ service
    FaqService,
    DynFaqService,
    FaqItem,
};
