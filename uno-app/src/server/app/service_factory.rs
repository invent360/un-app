//! Service factory for dependency injection

use crate::server::db::ConnectionPool;
use file_storage::{create_client_from_env, DynFileStorageClient};
use crate::server::geoip::OptionalGeoIp;
use crate::server::repositories::{
    AuditRepositoryImpl, ClaimRepositoryImpl, ContentItemRepositoryImpl, ContentRepositoryImpl,
    DynAuditRepository, DynClaimRepository, DynContentItemRepository, DynRbacRepository,
    DynReferralRepository, DynReviewRepository, DynSchemaRepository, DynStatsRepository,
    DynTestimonialRepository, DynSessionRepository, DynLaunchGateRepository, DynImmutableAuditRepository,
    DynConsentRepository, DynOutboxRepository, DynServiceIdentityRepository,
    DynImportRepository, DynPublicationRepository, DynEligibilityRepository,
    DynAllocationRepository, DynCreditOrderRepository,
    DynPilotRepository, DynEvidenceRepository, DynRetentionRepository,
    FaqRepositoryImpl, PostgresLicenseRepository, RbacRepositoryImpl,
    ReferralRepositoryImpl, ReviewRepositoryImpl, SchemaRepositoryImpl, StatsRepositoryImpl,
    TestimonialRepositoryImpl, SessionRepositoryImpl, LaunchGateRepositoryImpl,
    ImmutableAuditRepositoryImpl, ConsentRepositoryImpl, OutboxRepositoryImpl,
    ServiceIdentityRepositoryImpl, ImportRepositoryImpl, PublicationRepositoryImpl,
    EligibilityRepositoryImpl, AllocationRepositoryImpl, CreditOrderRepositoryImpl,
    PilotRepositoryImpl, EvidenceRepositoryImpl, RetentionRepositoryImpl,
    // R3-04: Nonce repository for cross-replica replay prevention
    NonceRepositoryImpl,
    // Phase 7-8 repositories
    DynSupportRepository, SupportRepositoryImpl,
    DynCohortRepository, CohortRepositoryImpl,
    DynExitRepository, ExitRepositoryImpl,
    DynMarketRepository, MarketRepositoryImpl,
    DynOperatorMetricsRepository, OperatorMetricsRepositoryImpl,
    DynForecastRepository, ForecastRepositoryImpl,
    DynWebhookRepository, WebhookRepositoryImpl,
    DynCommunicationRepository, CommunicationRepositoryImpl,
    DynMediaAssetRepository, MediaAssetRepositoryImpl,
};
use crate::server::services::{
    AuditServiceImpl, ContentItemServiceImpl, ContentServiceImpl, FaqServiceImpl, LicenseService,
    RbacServiceImpl, SchemaServiceImpl, StatsServiceImpl, SessionService, LaunchGateService,
    ConsentService, JobService, ServiceIdentityService,
    // Phase 4 services
    ReservationServiceImpl, DynReservationService,
    OwnershipServiceImpl, DynOwnershipService,
    AgentServiceImpl, DynAgentService,
    LifecycleServiceImpl, DynLifecycleService,
    JourneyServiceImpl, DynJourneyService,
    // Phase 5 services
    SettlementService, DynSettlementService,
    // Phase 7-8 services
    ForecastServiceImpl, DynForecastService,
    WebhookServiceImpl, DynWebhookService,
    CommunicationServiceImpl, DynCommunicationService,
    MediaAssetServiceImpl, DynMediaAssetService,
    // Phase 9 services
    PilotServiceImpl, DynPilotService,
    // R3-17: Retention service
    RetentionServiceImpl, DynRetentionService,
};
use std::sync::Arc;
use uno_api::auth::ClientRegistry;
use uno_api::services::LicenseAdminService;
use uno_api::traits::LicenseRepository;

/// ServiceFactory centralizes dependency injection for all services
#[derive(Clone)]
pub struct ServiceFactory {
    pub pool: ConnectionPool,
    /// License service for claim operations (user-facing)
    pub license_service: LicenseService,
    /// Stats repository
    pub stats_repository: DynStatsRepository,
    /// FAQ service
    pub faq_service: FaqServiceImpl,
    /// Stats service
    pub stats_service: StatsServiceImpl,
    /// Content service for CMS operations (legacy)
    pub content_service: ContentServiceImpl,
    /// Referral repository for referral code validation
    pub referral_repository: DynReferralRepository,
    /// Claim repository for updating claim referrals
    pub claim_repository: DynClaimRepository,
    /// Review repository for content review workflow
    pub review_repository: DynReviewRepository,
    /// Audit service for CMS action logging
    pub audit_service: AuditServiceImpl,
    /// RBAC service for permission management
    pub rbac_service: RbacServiceImpl,
    /// Schema service for schema-driven CMS
    pub schema_service: SchemaServiceImpl,
    /// Content item service for schema-driven CMS
    pub content_item_service: ContentItemServiceImpl,
    /// Testimonial repository for home page testimonials
    pub testimonial_repository: DynTestimonialRepository,
    /// uno-api license admin service (admin operations)
    pub license_admin_service: Arc<LicenseAdminService>,
    /// Client registry for HMAC authentication
    pub client_registry: Arc<ClientRegistry>,
    /// R3-04: Nonce repository for cross-replica replay prevention
    /// Implements uno_api::auth::AsyncNonceChecker for PostgreSQL-backed nonce storage
    pub nonce_repository: Arc<NonceRepositoryImpl>,
    /// GeoIP lookup service
    pub geoip: OptionalGeoIp,
    /// Session service for identity and session management (P2-01)
    pub session_service: SessionService,
    /// Launch gate service for feature toggles (P2-07)
    pub launch_gate_service: LaunchGateService,
    /// Session repository for direct access
    pub session_repository: DynSessionRepository,
    /// Launch gate repository for direct access
    pub launch_gate_repository: DynLaunchGateRepository,
    /// Consent service for privacy and consent management (P2-06)
    pub consent_service: ConsentService,
    /// Consent repository for direct access
    pub consent_repository: DynConsentRepository,
    /// Job service for durable work processing (P3-01)
    pub job_service: JobService,
    /// Outbox repository for transactional event publishing (P3-01)
    pub outbox_repository: DynOutboxRepository,
    /// Service identity service for M2M authentication (P3-03)
    pub service_identity_service: ServiceIdentityService,
    /// Service identity repository for direct access (P3-03)
    pub service_identity_repository: DynServiceIdentityRepository,
    // Phase 4 repositories
    /// Import repository for CSV/API imports (P4-01)
    pub import_repository: DynImportRepository,
    /// Publication repository for license publication workflow (P4-02)
    pub publication_repository: DynPublicationRepository,
    /// Eligibility repository for country/device/task rules (P4-03)
    pub eligibility_repository: DynEligibilityRepository,
    // Phase 4 services
    /// Reservation service for license reservation with eligibility (P4-04)
    pub reservation_service: DynReservationService,
    /// Ownership service for license ownership and gate validation (P4-05)
    pub ownership_service: DynOwnershipService,
    /// Agent service for agent approval/suspension workflow (P4-06)
    pub agent_service: DynAgentService,
    /// Lifecycle service for cancel/expiry/release tracking (P4-07)
    pub lifecycle_service: DynLifecycleService,
    /// Journey service for onboarding progress tracking (R3-13)
    pub journey_service: DynJourneyService,
    // Phase 5 repositories and services
    /// Allocation repository for finance ledger (P5-02)
    pub allocation_repository: Option<DynAllocationRepository>,
    /// Credit order repository for funding lifecycle (P5-03)
    pub credit_order_repository: Option<DynCreditOrderRepository>,
    /// Settlement service for finance operations (P5-04)
    pub settlement_service: Option<DynSettlementService>,
    // Phase 7-8 repositories
    /// Support repository for ticket management (P7-01)
    pub support_repository: DynSupportRepository,
    /// Cohort repository for participant tracking (P7-02)
    pub cohort_repository: DynCohortRepository,
    /// Exit repository for voluntary exit management (P7-03)
    pub exit_repository: DynExitRepository,
    /// Market repository for market configuration (P7-04)
    pub market_repository: DynMarketRepository,
    /// Operator metrics repository for dashboard (P8-01)
    pub operator_metrics_repository: DynOperatorMetricsRepository,
    /// Forecast repository for scenario persistence (P8-02)
    pub forecast_repository: DynForecastRepository,
    /// Webhook repository for integration config (P8-03)
    pub webhook_repository: DynWebhookRepository,
    /// Communication repository for preferences (P8-04)
    pub communication_repository: DynCommunicationRepository,
    /// Media asset repository for file metadata (P6-01)
    pub media_asset_repository: DynMediaAssetRepository,
    // Phase 7-8 services
    /// Forecast service for scenario calculations (P8-02)
    pub forecast_service: DynForecastService,
    /// Webhook service for outbound integration (P8-03)
    pub webhook_service: DynWebhookService,
    /// Communication service for notifications (P8-04)
    pub communication_service: DynCommunicationService,
    /// Media asset service for file operations (P6-01)
    pub media_asset_service: DynMediaAssetService,
    // Phase 9 repositories and services
    /// Pilot repository for controlled rollout tracking (P9-08)
    pub pilot_repository: Option<DynPilotRepository>,
    /// Evidence repository for gate evidence requirements (P9-07)
    pub evidence_repository: Option<DynEvidenceRepository>,
    /// Pilot service for pilot participant management (P9-08)
    pub pilot_service: Option<DynPilotService>,
    /// R3-11: Centralized storage client for file operations
    pub storage_client: Option<DynFileStorageClient>,
    // R3-17: Data retention
    /// Retention repository for policy management
    pub retention_repository: DynRetentionRepository,
    /// Retention service for cleanup enforcement
    pub retention_service: DynRetentionService,
    /// Immutable audit repository (kept for background services)
    pub immutable_audit_repository: DynImmutableAuditRepository,
}

impl ServiceFactory {
    /// Create a new ServiceFactory with all dependencies wired up
    pub fn new(pool: ConnectionPool) -> Self {
        // Create the license repository (implements uno-api trait)
        let license_repo: Arc<dyn LicenseRepository + Send + Sync> =
            Arc::new(PostgresLicenseRepository::new(pool.clone()));

        // Create other repositories
        let stats_repository: DynStatsRepository = Arc::new(StatsRepositoryImpl::new(pool.clone()));
        let faq_repository = Arc::new(FaqRepositoryImpl::new(pool.clone()));
        let content_repository = Arc::new(ContentRepositoryImpl::new(pool.clone()));
        let referral_repository: DynReferralRepository =
            Arc::new(ReferralRepositoryImpl::new(pool.clone()));
        let claim_repository: DynClaimRepository = Arc::new(ClaimRepositoryImpl::new(pool.clone()));
        let review_repository: DynReviewRepository =
            Arc::new(ReviewRepositoryImpl::new(pool.clone()));
        let audit_repository: DynAuditRepository = Arc::new(AuditRepositoryImpl::new(pool.clone()));
        let rbac_repository: DynRbacRepository = Arc::new(RbacRepositoryImpl::new(pool.clone()));
        let schema_repository: DynSchemaRepository =
            Arc::new(SchemaRepositoryImpl::new(pool.clone()));
        let content_item_repository: DynContentItemRepository =
            Arc::new(ContentItemRepositoryImpl::new(pool.clone()));
        let testimonial_repository: DynTestimonialRepository =
            Arc::new(TestimonialRepositoryImpl::new(pool.clone()));

        // Create session and launch gate repositories (Phase 2)
        let session_repository: DynSessionRepository =
            Arc::new(SessionRepositoryImpl::new(pool.clone()));
        let launch_gate_repository: DynLaunchGateRepository =
            Arc::new(LaunchGateRepositoryImpl::new(pool.clone()));
        let immutable_audit_repository: DynImmutableAuditRepository =
            Arc::new(ImmutableAuditRepositoryImpl::new(pool.clone()));

        // Create license service for user-facing claim operations
        let license_service = LicenseService::new(license_repo.clone());

        // Create uno-api admin service for admin operations
        let license_admin_service = Arc::new(LicenseAdminService::new(license_repo));

        // Create FAQ service
        let faq_service = FaqServiceImpl::new(faq_repository);

        // Create stats service
        let stats_service = StatsServiceImpl::new(stats_repository.clone());

        // Create audit service for CMS action logging
        let audit_service = AuditServiceImpl::new(audit_repository.clone());

        // Create RBAC service for permission management
        let rbac_service = RbacServiceImpl::new(rbac_repository);

        // Create content service for CMS operations (with audit logging) - legacy
        let content_service =
            ContentServiceImpl::with_audit(content_repository, audit_service.clone());

        // Create schema service for schema-driven CMS
        let schema_service =
            SchemaServiceImpl::new(schema_repository.clone(), Some(audit_repository.clone()));

        // Create content item service for schema-driven CMS
        let content_item_service = ContentItemServiceImpl::new(
            content_item_repository,
            schema_repository,
            Some(audit_repository),
        );

        // Create session service for identity and session management (P2-01)
        let session_service = SessionService::new(
            session_repository.clone(),
            immutable_audit_repository.clone(),
        );

        // Create launch gate service for feature toggles (P2-07)
        let launch_gate_service = LaunchGateService::new(
            launch_gate_repository.clone(),
            immutable_audit_repository.clone(),
        );

        // Create consent repository and service (P2-06)
        let consent_repository: DynConsentRepository =
            Arc::new(ConsentRepositoryImpl::new(pool.clone()));
        let consent_service = ConsentService::new(
            consent_repository.clone(),
            immutable_audit_repository.clone(),
        );

        // Create outbox repository and job service (P3-01)
        let outbox_repository: DynOutboxRepository =
            Arc::new(OutboxRepositoryImpl::new(pool.clone()));
        let job_service = JobService::new(
            pool.clone(),
            outbox_repository.clone(),
            immutable_audit_repository.clone(),
        );

        // Create service identity repository and service (P3-03)
        let service_identity_repository: DynServiceIdentityRepository =
            Arc::new(ServiceIdentityRepositoryImpl::new(pool.clone()));
        let service_identity_service = ServiceIdentityService::new(
            service_identity_repository.clone(),
            immutable_audit_repository.clone(),
        );

        // Create Phase 4 repositories
        let import_repository: DynImportRepository =
            Arc::new(ImportRepositoryImpl::new(pool.clone()));
        let publication_repository: DynPublicationRepository =
            Arc::new(PublicationRepositoryImpl::new(pool.clone()));
        let eligibility_repository: DynEligibilityRepository =
            Arc::new(EligibilityRepositoryImpl::new(pool.clone()));

        // Create Phase 4 services
        let reservation_service: DynReservationService = Arc::new(ReservationServiceImpl::new(
            pool.clone(),
            claim_repository.clone(),
            eligibility_repository.clone(),
        ));
        let ownership_service: DynOwnershipService = Arc::new(OwnershipServiceImpl::new(
            pool.clone(),
            launch_gate_repository.clone(),
        ));
        let agent_service: DynAgentService = Arc::new(AgentServiceImpl::new(pool.clone()));
        let lifecycle_service: DynLifecycleService =
            Arc::new(LifecycleServiceImpl::new(pool.clone()));

        // R3-13: Journey service for onboarding progress tracking
        let journey_service: DynJourneyService = Arc::new(JourneyServiceImpl::new(
            pool.clone(),
            immutable_audit_repository.clone(),
        ));

        // Create Phase 5 repositories and services
        let allocation_repository: DynAllocationRepository =
            Arc::new(AllocationRepositoryImpl::new(pool.clone()));
        let credit_order_repository: DynCreditOrderRepository =
            Arc::new(CreditOrderRepositoryImpl::new(pool.clone()));
        let settlement_service: DynSettlementService = Arc::new(SettlementService::new(
            allocation_repository.clone(),
            credit_order_repository.clone(),
            immutable_audit_repository.clone(),
            outbox_repository.clone(),
        ));

        // Create Phase 7-8 repositories
        let support_repository: DynSupportRepository =
            Arc::new(SupportRepositoryImpl::new(pool.clone()));
        let cohort_repository: DynCohortRepository =
            Arc::new(CohortRepositoryImpl::new(pool.clone()));
        let exit_repository: DynExitRepository =
            Arc::new(ExitRepositoryImpl::new(pool.clone()));
        let market_repository: DynMarketRepository =
            Arc::new(MarketRepositoryImpl::new(pool.clone()));
        let operator_metrics_repository: DynOperatorMetricsRepository =
            Arc::new(OperatorMetricsRepositoryImpl::new(pool.clone()));
        let forecast_repository: DynForecastRepository =
            Arc::new(ForecastRepositoryImpl::new(pool.clone()));
        let webhook_repository: DynWebhookRepository =
            Arc::new(WebhookRepositoryImpl::new(pool.clone()));
        let communication_repository: DynCommunicationRepository =
            Arc::new(CommunicationRepositoryImpl::new(pool.clone()));
        let media_asset_repository: DynMediaAssetRepository =
            Arc::new(MediaAssetRepositoryImpl::new(pool.clone()));

        // Create Phase 7-8 services
        let forecast_service: DynForecastService =
            Arc::new(ForecastServiceImpl::new(forecast_repository.clone()));
        let webhook_service: DynWebhookService =
            Arc::new(WebhookServiceImpl::new(webhook_repository.clone()));
        let communication_service: DynCommunicationService =
            Arc::new(CommunicationServiceImpl::new(communication_repository.clone()));
        let media_asset_service: DynMediaAssetService =
            Arc::new(MediaAssetServiceImpl::new(media_asset_repository.clone()));

        // Create Phase 9 repositories and services
        let pilot_repository: DynPilotRepository =
            Arc::new(PilotRepositoryImpl::new(pool.clone()));
        let evidence_repository: DynEvidenceRepository =
            Arc::new(EvidenceRepositoryImpl::new(pool.clone()));
        let pilot_service: DynPilotService = Arc::new(PilotServiceImpl::new(
            pilot_repository.clone(),
            launch_gate_repository.clone(),
            immutable_audit_repository.clone(),
        ));

        // R3-17: Create retention repository and service
        let retention_repository: DynRetentionRepository =
            Arc::new(RetentionRepositoryImpl::new(pool.clone()));
        let retention_service: DynRetentionService = Arc::new(RetentionServiceImpl::new(
            retention_repository.clone(),
            immutable_audit_repository.clone(),
        ));

        // Create client registry with admin credentials from environment
        let client_registry = Arc::new(Self::create_client_registry());

        // R3-04: Create nonce repository for cross-replica replay prevention
        let nonce_repository = Arc::new(NonceRepositoryImpl::new(pool.clone()));

        // Initialize GeoIP service from environment
        let geoip = OptionalGeoIp::from_env();

        // R3-11: Create centralized storage client (once, shared across handlers)
        let storage_client = match create_client_from_env() {
            Ok(client) => {
                tracing::info!("Storage client initialized successfully");
                Some(client)
            }
            Err(e) => {
                tracing::warn!("Storage client initialization failed (file operations will use per-request clients): {}", e);
                None
            }
        };

        ServiceFactory {
            pool,
            license_service,
            stats_repository,
            faq_service,
            stats_service,
            content_service,
            referral_repository,
            claim_repository,
            review_repository,
            audit_service,
            rbac_service,
            schema_service,
            content_item_service,
            testimonial_repository,
            license_admin_service,
            client_registry,
            nonce_repository,
            geoip,
            session_service,
            launch_gate_service,
            session_repository,
            launch_gate_repository,
            consent_service,
            consent_repository,
            job_service,
            outbox_repository,
            service_identity_service,
            service_identity_repository,
            // Phase 4
            import_repository,
            publication_repository,
            eligibility_repository,
            reservation_service,
            ownership_service,
            agent_service,
            lifecycle_service,
            journey_service,
            // Phase 5
            allocation_repository: Some(allocation_repository),
            credit_order_repository: Some(credit_order_repository),
            settlement_service: Some(settlement_service),
            // Phase 7-8 repositories
            support_repository,
            cohort_repository,
            exit_repository,
            market_repository,
            operator_metrics_repository,
            forecast_repository,
            webhook_repository,
            communication_repository,
            media_asset_repository,
            // Phase 7-8 services
            forecast_service,
            webhook_service,
            communication_service,
            media_asset_service,
            // Phase 9
            pilot_repository: Some(pilot_repository),
            evidence_repository: Some(evidence_repository),
            pilot_service: Some(pilot_service),
            // R3-11: Centralized storage client
            storage_client,
            // R3-17: Retention
            retention_repository,
            retention_service,
            immutable_audit_repository,
        }
    }

    /// Create client registry with credentials from environment
    fn create_client_registry() -> ClientRegistry {
        let registry = ClientRegistry::new();

        // Load admin client credentials from environment
        if let (Ok(client_id), Ok(secret_key)) = (
            std::env::var("ADMIN_CLIENT_ID"),
            std::env::var("ADMIN_SECRET_KEY"),
        ) {
            registry.register(&client_id, secret_key.as_bytes(), &["admin"]);
            tracing::info!("Registered admin client: {}", client_id);
        } else {
            tracing::warn!(
                "ADMIN_CLIENT_ID or ADMIN_SECRET_KEY not set - admin API will be unavailable"
            );
        }

        registry
    }

    /// R3-11: Verify media volume is mounted and writable
    ///
    /// This should be called during startup to fail fast if storage is misconfigured.
    /// Checks:
    /// 1. Volume path exists and is a directory
    /// 2. Marker file exists (in production mode)
    /// 3. Write access is available
    pub fn verify_volume() -> Result<(), String> {
        let mount_path = std::env::var("FILE_STORAGE_LOCAL_PATH")
            .or_else(|_| std::env::var("MEDIA_VOLUME_PATH"))
            .unwrap_or_else(|_| "/var/lib/uno-app/media".into());

        let path = std::path::Path::new(&mount_path);

        // Check directory exists
        if !path.exists() {
            return Err(format!("Media volume not mounted: {} does not exist", mount_path));
        }

        if !path.is_dir() {
            return Err(format!("Media volume path is not a directory: {}", mount_path));
        }

        // Check marker file in production
        let is_production = std::env::var("PRODUCTION")
            .or_else(|_| std::env::var("APP_ENV"))
            .map(|v| v == "production" || v == "true" || v == "1")
            .unwrap_or(false);

        if is_production {
            let marker_path = path.join(".uno-volume");
            if !marker_path.exists() {
                return Err(format!(
                    "Media volume marker file missing: {}/.uno-volume",
                    mount_path
                ));
            }

            // Verify marker content if FILE_STORAGE_VOLUME_ID is set
            if let Ok(expected_id) = std::env::var("FILE_STORAGE_VOLUME_ID") {
                match std::fs::read_to_string(&marker_path) {
                    Ok(content) => {
                        if content.trim() != expected_id.trim() {
                            return Err(format!(
                                "Media volume identity mismatch: expected '{}', got '{}'",
                                expected_id.trim(),
                                content.trim()
                            ));
                        }
                    }
                    Err(e) => {
                        return Err(format!("Failed to read marker file: {}", e));
                    }
                }
            }
        }

        // Verify write access
        let test_path = path.join(".write-test");
        if let Err(e) = std::fs::write(&test_path, b"test") {
            return Err(format!("Media volume not writable: {}", e));
        }
        if let Err(e) = std::fs::remove_file(&test_path) {
            tracing::warn!("Failed to clean up write test file: {}", e);
        }

        tracing::info!("Media volume verified: {}", mount_path);
        Ok(())
    }
}
