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

// Re-export implementations
pub use license_repository::PostgresLicenseRepository;
pub use stats_repository::StatsRepositoryImpl;
pub use faq_repository::FaqRepositoryImpl;
pub use content_repository::ContentRepositoryImpl;
pub use referral_repository::ReferralRepositoryImpl;
pub use claim_repository::ClaimRepositoryImpl;
pub use review_repository::ReviewRepositoryImpl;
pub use audit_repository::AuditRepositoryImpl;
pub use rbac_repository::RbacRepositoryImpl;
pub use schema_repository::SchemaRepositoryImpl;
pub use content_item_repository::ContentItemRepositoryImpl;
pub use testimonial_repository::TestimonialRepositoryImpl;

// Re-export dynamic type aliases
pub use license_repository::DynLicenseRepository;
pub use stats_repository::DynStatsRepository;
pub use faq_repository::DynFaqRepository;
pub use content_repository::DynContentRepository;
pub use referral_repository::DynReferralRepository;
pub use claim_repository::DynClaimRepository;
pub use review_repository::DynReviewRepository;
pub use audit_repository::DynAuditRepository;
pub use rbac_repository::DynRbacRepository;
pub use schema_repository::DynSchemaRepository;
pub use content_item_repository::DynContentItemRepository;
pub use testimonial_repository::DynTestimonialRepository;

// Re-export repository traits from stats and faq (license uses uno-api trait)
pub use stats_repository::StatsRepository;
pub use faq_repository::FaqRepository;
pub use content_repository::ContentRepository;
pub use referral_repository::{ReferralRepository, ReferralInput};
pub use claim_repository::ClaimRepository;
pub use review_repository::ReviewRepository;
pub use audit_repository::AuditRepository;
pub use rbac_repository::RbacRepository;
pub use schema_repository::SchemaRepository;
pub use content_item_repository::ContentItemRepository;
pub use testimonial_repository::{TestimonialRepository, Testimonial};
