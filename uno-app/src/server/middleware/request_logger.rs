//! Production-grade request/response logging middleware
//!
//! Provides structured logging for all API requests with:
//! - Unique request IDs for correlation
//! - Request method, path, query parameters
//! - Response status and duration
//! - Client IP and User-Agent
//! - Request/response body size
//! - Error details for failed requests

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error, HttpMessage,
};
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::time::Instant;
use tracing::{debug, error, info, info_span, warn, Instrument, Span};
use uuid::Uuid;

/// Header name for request ID
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Request logging configuration
#[derive(Clone)]
pub struct RequestLoggerConfig {
    /// Log request bodies (careful with sensitive data)
    pub log_request_body: bool,
    /// Log response bodies (careful with sensitive data)
    pub log_response_body: bool,
    /// Maximum body size to log (bytes)
    pub max_body_log_size: usize,
    /// Paths to exclude from detailed logging
    pub excluded_paths: Vec<String>,
    /// Log level for successful requests
    pub success_level: LogLevel,
    /// Log level for client errors (4xx)
    pub client_error_level: LogLevel,
    /// Log level for server errors (5xx)
    pub server_error_level: LogLevel,
}

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for RequestLoggerConfig {
    fn default() -> Self {
        Self {
            log_request_body: false,
            log_response_body: false,
            max_body_log_size: 4096,
            excluded_paths: vec![
                "/pkg".to_string(),
                "/assets".to_string(),
                "/favicon.ico".to_string(),
            ],
            success_level: LogLevel::Info,
            client_error_level: LogLevel::Warn,
            server_error_level: LogLevel::Error,
        }
    }
}

impl RequestLoggerConfig {
    /// Create a new config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable request body logging
    pub fn with_request_body(mut self) -> Self {
        self.log_request_body = true;
        self
    }

    /// Enable response body logging
    pub fn with_response_body(mut self) -> Self {
        self.log_response_body = true;
        self
    }

    /// Set paths to exclude from logging
    pub fn exclude_paths(mut self, paths: Vec<&str>) -> Self {
        self.excluded_paths = paths.into_iter().map(String::from).collect();
        self
    }

    /// Add paths to exclude from logging
    pub fn add_excluded_path(mut self, path: &str) -> Self {
        self.excluded_paths.push(path.to_string());
        self
    }
}

/// Request logger middleware factory
pub struct RequestLogger {
    config: RequestLoggerConfig,
}

impl RequestLogger {
    /// Create a new request logger with default config
    pub fn new() -> Self {
        Self {
            config: RequestLoggerConfig::default(),
        }
    }

    /// Create a new request logger with custom config
    pub fn with_config(config: RequestLoggerConfig) -> Self {
        Self { config }
    }

    /// Production config - logs all API requests with appropriate levels
    pub fn production() -> Self {
        Self {
            config: RequestLoggerConfig {
                log_request_body: false,
                log_response_body: false,
                max_body_log_size: 1024,
                excluded_paths: vec![
                    "/pkg".to_string(),
                    "/assets".to_string(),
                    "/favicon.ico".to_string(),
                    "/.well-known".to_string(),
                ],
                success_level: LogLevel::Info,
                client_error_level: LogLevel::Warn,
                server_error_level: LogLevel::Error,
            },
        }
    }

    /// Development config - more verbose logging
    pub fn development() -> Self {
        Self {
            config: RequestLoggerConfig {
                log_request_body: true,
                log_response_body: true,
                max_body_log_size: 4096,
                excluded_paths: vec![
                    "/pkg".to_string(),
                    "/assets".to_string(),
                ],
                success_level: LogLevel::Debug,
                client_error_level: LogLevel::Info,
                server_error_level: LogLevel::Warn,
            },
        }
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware {
            service,
            config: self.config.clone(),
        }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
    config: RequestLoggerConfig,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
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

        // Generate or extract request ID
        let request_id = req
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // Store request ID in extensions for handlers to access
        req.extensions_mut().insert(RequestId(request_id.clone()));

        // Extract request info
        let method = req.method().to_string();
        let path = req.path().to_string();
        let query = req.query_string().to_string();
        let version = format!("{:?}", req.version());

        let client_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let user_agent = req
            .headers()
            .get(actix_web::http::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .unwrap_or_else(|| "-".to_string());

        let content_length = req
            .headers()
            .get(actix_web::http::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        let content_type = req
            .headers()
            .get(actix_web::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        // Check if path should be excluded
        let is_excluded = self.config.excluded_paths.iter().any(|p| path.starts_with(p));
        let config = self.config.clone();

        // Create tracing span for the request
        let span = info_span!(
            "http_request",
            request_id = %request_id,
            method = %method,
            path = %path,
            client_ip = %client_ip,
        );

        // Log request start
        if !is_excluded {
            debug!(
                parent: &span,
                query = %query,
                user_agent = %user_agent,
                content_length = %content_length,
                content_type = ?content_type,
                "Request started"
            );
        }

        let fut = self.service.call(req);

        Box::pin(
            async move {
                let result = fut.await;
                let duration = start.elapsed();
                let duration_ms = duration.as_secs_f64() * 1000.0;

                match &result {
                    Ok(res) => {
                        let status = res.status();
                        let status_code = status.as_u16();

                        // Add request ID to response headers
                        // Note: We can't modify headers here directly, but handlers can access RequestId

                        if !is_excluded {
                            // Determine response size if available
                            let response_size = res
                                .headers()
                                .get(actix_web::http::header::CONTENT_LENGTH)
                                .and_then(|v| v.to_str().ok())
                                .and_then(|s| s.parse::<usize>().ok());

                            // Log based on status code
                            match status_code {
                                200..=299 => {
                                    match config.success_level {
                                        LogLevel::Debug => debug!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            response_size = ?response_size,
                                            "Request completed"
                                        ),
                                        LogLevel::Info => info!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            response_size = ?response_size,
                                            "Request completed"
                                        ),
                                        _ => info!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            "Request completed"
                                        ),
                                    }
                                }
                                300..=399 => {
                                    let location = res
                                        .headers()
                                        .get(actix_web::http::header::LOCATION)
                                        .and_then(|v| v.to_str().ok());
                                    info!(
                                        status = %status_code,
                                        duration_ms = %format!("{:.2}", duration_ms),
                                        redirect_to = ?location,
                                        "Request redirected"
                                    );
                                }
                                400..=499 => {
                                    match config.client_error_level {
                                        LogLevel::Debug => debug!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            reason = %status.canonical_reason().unwrap_or("Unknown"),
                                            "Client error"
                                        ),
                                        LogLevel::Info => info!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            reason = %status.canonical_reason().unwrap_or("Unknown"),
                                            "Client error"
                                        ),
                                        LogLevel::Warn => warn!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            reason = %status.canonical_reason().unwrap_or("Unknown"),
                                            "Client error"
                                        ),
                                        LogLevel::Error => error!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            reason = %status.canonical_reason().unwrap_or("Unknown"),
                                            "Client error"
                                        ),
                                    }
                                }
                                500..=599 => {
                                    match config.server_error_level {
                                        LogLevel::Warn => warn!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            reason = %status.canonical_reason().unwrap_or("Unknown"),
                                            "Server error"
                                        ),
                                        _ => error!(
                                            status = %status_code,
                                            duration_ms = %format!("{:.2}", duration_ms),
                                            reason = %status.canonical_reason().unwrap_or("Unknown"),
                                            "Server error"
                                        ),
                                    }
                                }
                                _ => {
                                    info!(
                                        status = %status_code,
                                        duration_ms = %format!("{:.2}", duration_ms),
                                        "Request completed"
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            error = %e,
                            duration_ms = %format!("{:.2}", duration_ms),
                            "Request failed"
                        );
                    }
                }

                result
            }
            .instrument(span),
        )
    }
}

/// Request ID wrapper for storing in request extensions
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

impl RequestId {
    /// Get the request ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Extract request ID from request extensions
pub fn get_request_id(req: &ServiceRequest) -> Option<String> {
    req.extensions().get::<RequestId>().map(|id| id.0.clone())
}

/// Initialize the logging system with production-grade configuration
pub fn init_logging() {
    use tracing_subscriber::{
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        util::SubscriberInitExt,
        EnvFilter,
    };

    // Get log level from environment or default to info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default filter: info for app, warn for dependencies
        EnvFilter::new("warn,uno_app=info,actix_web=info,actix_server=info")
    });

    // Determine if we should use JSON format (default) or pretty format (development)
    let use_json = std::env::var("LOG_FORMAT")
        .map(|v| v.to_lowercase() != "pretty")
        .unwrap_or(true);

    if use_json {
        // JSON format for production (structured logging)
        let fmt_layer = fmt::layer()
            .json()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    } else {
        // Pretty format for development
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .with_span_events(FmtSpan::NONE)
            .compact();

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    }

    info!("Logging initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_generation() {
        let id = RequestId(Uuid::new_v4().to_string());
        assert!(!id.0.is_empty());
        assert_eq!(id.0.len(), 36); // UUID format
    }

    #[test]
    fn test_config_builder() {
        let config = RequestLoggerConfig::new()
            .with_request_body()
            .with_response_body()
            .exclude_paths(vec!["/health", "/ready"]);

        assert!(config.log_request_body);
        assert!(config.log_response_body);
        assert!(config.excluded_paths.contains(&"/health".to_string()));
    }
}
