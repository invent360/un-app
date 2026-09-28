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
