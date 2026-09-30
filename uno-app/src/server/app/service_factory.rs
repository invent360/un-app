//! Service factory for dependency injection

use crate::server::db::ConnectionPool;
use crate::server::geoip::OptionalGeoIp;
use crate::server::repositories::{
    AuditRepositoryImpl, ClaimRepositoryImpl, ContentItemRepositoryImpl, ContentRepositoryImpl,
    DynAuditRepository, DynClaimRepository, DynContentItemRepository, DynRbacRepository,
    DynReferralRepository, DynReviewRepository, DynSchemaRepository, DynStatsRepository,
    DynTestimonialRepository, DynSessionRepository, DynLaunchGateRepository, DynImmutableAuditRepository,
    DynConsentRepository, DynOutboxRepository, DynServiceIdentityRepository,
    DynImportRepository, DynPublicationRepository, DynEligibilityRepository,
    FaqRepositoryImpl, PostgresLicenseRepository, RbacRepositoryImpl,
    ReferralRepositoryImpl, ReviewRepositoryImpl, SchemaRepositoryImpl, StatsRepositoryImpl,
    TestimonialRepositoryImpl, SessionRepositoryImpl, LaunchGateRepositoryImpl,
    ImmutableAuditRepositoryImpl, ConsentRepositoryImpl, OutboxRepositoryImpl,
    ServiceIdentityRepositoryImpl, ImportRepositoryImpl, PublicationRepositoryImpl,
    EligibilityRepositoryImpl,
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

        // Create client registry with admin credentials from environment
        let client_registry = Arc::new(Self::create_client_registry());

        // Initialize GeoIP service from environment
        let geoip = OptionalGeoIp::from_env();

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
}
