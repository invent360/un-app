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
mod outbox_publisher;
mod inbox_processor;

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
    run_lease_reclaimer, run_event_cleanup,
};
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
};
pub use outbox_publisher::{
    OutboxPublisher, OutboxPublisherConfig, PublishStats, WebhookPayload,
    EventPublisher, DynEventPublisher, WebhookPublisher, NullPublisher,
};
pub use inbox_processor::{
    InboxProcessor, InboxOutcome, InboxContext,
    InboxHandler, DynInboxHandler, LoggingHandler,
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
