//! Service factory for dependency injection

use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::server::geoip::OptionalGeoIp;
use crate::server::repositories::{
    PostgresLicenseRepository, StatsRepositoryImpl, DynStatsRepository, FaqRepositoryImpl,
    ContentRepositoryImpl,
    ReferralRepositoryImpl, DynReferralRepository,
    ClaimRepositoryImpl, DynClaimRepository,
    ReviewRepositoryImpl, DynReviewRepository,
    AuditRepositoryImpl, DynAuditRepository,
    RbacRepositoryImpl, DynRbacRepository,
    SchemaRepositoryImpl, DynSchemaRepository,
    ContentItemRepositoryImpl, DynContentItemRepository,
    TestimonialRepositoryImpl, DynTestimonialRepository,
};
use crate::server::services::{
    LicenseService, FaqServiceImpl, StatsServiceImpl, ContentServiceImpl, AuditServiceImpl,
    RbacServiceImpl, SchemaServiceImpl, ContentItemServiceImpl,
};
use uno_api::services::LicenseAdminService;
use uno_api::auth::ClientRegistry;
use uno_api::traits::LicenseRepository;

/// ServiceFactory centralizes dependency injection for all services
#[derive(Clone)]
pub struct ServiceFactory {
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
        let referral_repository: DynReferralRepository = Arc::new(ReferralRepositoryImpl::new(pool.clone()));
        let claim_repository: DynClaimRepository = Arc::new(ClaimRepositoryImpl::new(pool.clone()));
        let review_repository: DynReviewRepository = Arc::new(ReviewRepositoryImpl::new(pool.clone()));
        let audit_repository: DynAuditRepository = Arc::new(AuditRepositoryImpl::new(pool.clone()));
        let rbac_repository: DynRbacRepository = Arc::new(RbacRepositoryImpl::new(pool.clone()));
        let schema_repository: DynSchemaRepository = Arc::new(SchemaRepositoryImpl::new(pool.clone()));
        let content_item_repository: DynContentItemRepository = Arc::new(ContentItemRepositoryImpl::new(pool.clone()));
        let testimonial_repository: DynTestimonialRepository = Arc::new(TestimonialRepositoryImpl::new(pool.clone()));

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
        let content_service = ContentServiceImpl::with_audit(content_repository, audit_service.clone());

        // Create schema service for schema-driven CMS
        let schema_service = SchemaServiceImpl::new(schema_repository.clone(), Some(audit_repository.clone()));

        // Create content item service for schema-driven CMS
        let content_item_service = ContentItemServiceImpl::new(
            content_item_repository,
            schema_repository,
            Some(audit_repository),
        );

        // Create client registry with admin credentials from environment
        let client_registry = Arc::new(Self::create_client_registry());

        // Initialize GeoIP service from environment
        let geoip = OptionalGeoIp::from_env();

        ServiceFactory {
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
            tracing::warn!("ADMIN_CLIENT_ID or ADMIN_SECRET_KEY not set - admin API will be unavailable");
        }

        registry
    }
}
