//! Actix middleware

pub mod audit_middleware;
pub mod auth_middleware;
pub mod compression_middleware;
pub mod cors_middleware;
pub mod csrf_middleware;
pub mod rate_limit_middleware;
pub mod rbac_middleware;
pub mod request_logger;
pub mod security_middleware;
pub mod validation_middleware;
pub mod visitor_middleware;

pub use audit_middleware::{AuditLogger, AuditLogEntry, SecurityEventType, init_audit_logging};
pub use auth_middleware::AdminAuth;
pub use compression_middleware::{CompressionConfig, CompressionStats, is_compressible, add_vary_header};
pub use cors_middleware::CorsHeaders;
pub use csrf_middleware::CsrfProtection;
pub use rate_limit_middleware::{RateLimiter, RateLimitConfig, RateLimiterState};
pub use request_logger::{RequestLogger, RequestLoggerConfig, RequestId, init_logging};
pub use security_middleware::SecurityHeaders;
pub use validation_middleware::{
    RequestSizeLimit, ValidationErrorResponse, ValidationFieldError,
    validate_request, sanitize_string, sanitize_and_limit, patterns,
};
pub use visitor_middleware::VisitorTracker;
