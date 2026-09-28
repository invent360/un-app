//! Structured audit logging middleware for security events
//!
//! Logs all security-relevant events in a structured JSON format:
//! - Authentication attempts
//! - Authorization failures
//! - Rate limit hits
//! - CSRF violations
//! - Suspicious request patterns

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{Method, StatusCode},
    Error,
};
use serde::Serialize;
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::time::Instant;
use tracing::{info, warn, error, span, Level};

/// Security event types for audit logging
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    /// Successful request
    RequestCompleted,
    /// Authentication attempt
    AuthAttempt,
    /// Authentication success
    AuthSuccess,
    /// Authentication failure
    AuthFailure,
    /// Authorization denied
    AuthorizationDenied,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// CSRF validation failed
    CsrfViolation,
    /// Suspicious request pattern
    SuspiciousRequest,
    /// Input validation failure
    ValidationFailure,
    /// Server error
    ServerError,
}

/// Audit log entry
#[derive(Debug, Serialize)]
pub struct AuditLogEntry {
    /// Timestamp in ISO 8601 format
    pub timestamp: String,
    /// Event type
    pub event_type: SecurityEventType,
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Client IP address
    pub client_ip: String,
    /// User agent
    pub user_agent: Option<String>,
    /// Response status code
    pub status_code: u16,
    /// Request duration in milliseconds
    pub duration_ms: u64,
    /// Additional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl AuditLogEntry {
    /// Create a new audit log entry
    pub fn new(
        event_type: SecurityEventType,
        method: &Method,
        path: &str,
        client_ip: &str,
        status_code: StatusCode,
        duration_ms: u64,
    ) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type,
            method: method.to_string(),
            path: path.to_string(),
            client_ip: client_ip.to_string(),
            user_agent: None,
            status_code: status_code.as_u16(),
            duration_ms,
            context: None,
        }
    }

    /// Add user agent
    pub fn with_user_agent(mut self, user_agent: Option<String>) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Add additional context
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = Some(context);
        self
    }

    /// Log the entry
    pub fn log(&self) {
        match &self.event_type {
            SecurityEventType::RequestCompleted => {
                info!(
                    target: "audit",
                    event = ?self.event_type,
                    method = %self.method,
                    path = %self.path,
                    status = %self.status_code,
                    duration_ms = %self.duration_ms,
                    client_ip = %self.client_ip,
                    "Request completed"
                );
            }
            SecurityEventType::AuthFailure
            | SecurityEventType::AuthorizationDenied
            | SecurityEventType::CsrfViolation
            | SecurityEventType::SuspiciousRequest => {
                warn!(
                    target: "audit",
                    event = ?self.event_type,
                    method = %self.method,
                    path = %self.path,
                    status = %self.status_code,
                    client_ip = %self.client_ip,
                    context = ?self.context,
                    "Security event"
                );
            }
            SecurityEventType::RateLimitExceeded => {
                warn!(
                    target: "audit",
                    event = ?self.event_type,
                    method = %self.method,
                    path = %self.path,
                    client_ip = %self.client_ip,
                    "Rate limit exceeded"
                );
            }
            SecurityEventType::ServerError => {
                error!(
                    target: "audit",
                    event = ?self.event_type,
                    method = %self.method,
                    path = %self.path,
                    status = %self.status_code,
                    client_ip = %self.client_ip,
                    "Server error"
                );
            }
            _ => {
                info!(
                    target: "audit",
                    event = ?self.event_type,
                    method = %self.method,
                    path = %self.path,
                    status = %self.status_code,
                    client_ip = %self.client_ip,
                    "Audit event"
                );
            }
        }
    }
}

/// Audit logging middleware factory
pub struct AuditLogger {
    /// Log all requests or only security events
    log_all_requests: bool,
    /// Paths to exclude from logging
    excluded_paths: Vec<String>,
}

impl AuditLogger {
    /// Create new audit logger
    pub fn new() -> Self {
        Self {
            log_all_requests: false,
            excluded_paths: vec![
                "/health".to_string(),
                "/ready".to_string(),
                "/metrics".to_string(),
            ],
        }
    }

    /// Log all requests, not just security events
    pub fn log_all(mut self) -> Self {
        self.log_all_requests = true;
        self
    }

    /// Add paths to exclude from logging
    pub fn exclude_paths(mut self, paths: Vec<String>) -> Self {
        self.excluded_paths.extend(paths);
        self
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuditLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuditLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuditLoggerMiddleware {
            service,
            log_all_requests: self.log_all_requests,
            excluded_paths: self.excluded_paths.clone(),
        }))
    }
}

pub struct AuditLoggerMiddleware<S> {
    service: S,
    log_all_requests: bool,
    excluded_paths: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for AuditLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();

        let client_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let user_agent = req
            .headers()
            .get(actix_web::http::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let log_all = self.log_all_requests;
        let excluded = self.excluded_paths.iter().any(|p| path.starts_with(p));

        // Create a span for the request
        let request_span = span!(
            Level::INFO,
            "http_request",
            method = %method,
            path = %path,
            client_ip = %client_ip
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let _enter = request_span.enter();

            let res = fut.await?;
            let duration = start.elapsed();
            let status = res.status();

            // Determine event type based on status
            let event_type = match status.as_u16() {
                200..=299 => SecurityEventType::RequestCompleted,
                401 => SecurityEventType::AuthFailure,
                403 => {
                    // Could be CSRF or authorization
                    SecurityEventType::AuthorizationDenied
                }
                429 => SecurityEventType::RateLimitExceeded,
                422 => SecurityEventType::ValidationFailure,
                500..=599 => SecurityEventType::ServerError,
                _ => SecurityEventType::RequestCompleted,
            };

            // Log security events or all requests if enabled
            let should_log = !excluded && (log_all || !matches!(event_type, SecurityEventType::RequestCompleted));

            if should_log {
                let entry = AuditLogEntry::new(
                    event_type,
                    &method,
                    &path,
                    &client_ip,
                    status,
                    duration.as_millis() as u64,
                )
                .with_user_agent(user_agent);

                entry.log();
            }

            Ok(res)
        })
    }
}

/// Initialize the audit logging system
pub fn init_audit_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,audit=info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::Method;

    #[test]
    fn test_audit_log_entry_creation() {
        let entry = AuditLogEntry::new(
            SecurityEventType::AuthFailure,
            &Method::POST,
            "/api/login",
            "192.168.1.1",
            StatusCode::UNAUTHORIZED,
            150,
        );

        assert_eq!(entry.method, "POST");
        assert_eq!(entry.path, "/api/login");
        assert_eq!(entry.status_code, 401);
        assert_eq!(entry.duration_ms, 150);
    }

    #[test]
    fn test_audit_log_entry_with_context() {
        let entry = AuditLogEntry::new(
            SecurityEventType::SuspiciousRequest,
            &Method::GET,
            "/admin",
            "10.0.0.1",
            StatusCode::FORBIDDEN,
            50,
        )
        .with_context(serde_json::json!({
            "reason": "Multiple failed attempts"
        }));

        assert!(entry.context.is_some());
    }
}
