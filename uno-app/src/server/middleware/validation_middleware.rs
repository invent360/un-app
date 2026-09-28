//! Request validation middleware
//!
//! Provides request body validation using the validator crate.
//! Validates incoming JSON payloads against defined schemas.

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use validator::Validate;

/// Validation error response
#[derive(Debug, serde::Serialize)]
pub struct ValidationErrorResponse {
    pub error: String,
    pub code: String,
    pub details: Vec<ValidationFieldError>,
}

/// Individual field validation error
#[derive(Debug, serde::Serialize)]
pub struct ValidationFieldError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

/// Convert validator errors to our response format
pub fn format_validation_errors(errors: validator::ValidationErrors) -> Vec<ValidationFieldError> {
    let mut field_errors = Vec::new();

    for (field, errs) in errors.field_errors() {
        for err in errs {
            field_errors.push(ValidationFieldError {
                field: field.to_string(),
                message: err.message.clone().map(|c| c.to_string()).unwrap_or_else(|| {
                    format!("Validation failed for {}", field)
                }),
                code: err.code.to_string().into(),
            });
        }
    }

    field_errors
}

/// Validate a request body and return formatted errors
pub fn validate_request<T: Validate>(data: &T) -> Result<(), ValidationErrorResponse> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(errors) => Err(ValidationErrorResponse {
            error: "Validation failed".to_string(),
            code: "VALIDATION_ERROR".to_string(),
            details: format_validation_errors(errors),
        }),
    }
}

/// Common validation patterns
pub mod patterns {
    use regex::Regex;
    use std::sync::LazyLock;

    /// Email pattern
    pub static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    });

    /// UUID pattern
    pub static UUID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap()
    });

    /// Alphanumeric with underscores
    pub static ALPHANUMERIC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9_]+$").unwrap()
    });

    /// License key pattern (e.g., XXXX-XXXX-XXXX-XXXX)
    pub static LICENSE_KEY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[A-Z0-9]{4}-[A-Z0-9]{4}-[A-Z0-9]{4}-[A-Z0-9]{4}$").unwrap()
    });

    /// Safe filename pattern (no path traversal)
    pub static SAFE_FILENAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9_\-\.]+$").unwrap()
    });

    /// Validate string doesn't contain path traversal
    pub fn is_safe_path(path: &str) -> bool {
        !path.contains("..") &&
        !path.contains("//") &&
        !path.starts_with('/') &&
        !path.contains('\0')
    }

    /// Validate string doesn't contain script tags or event handlers
    pub fn is_xss_safe(input: &str) -> bool {
        let lower = input.to_lowercase();
        !lower.contains("<script") &&
        !lower.contains("javascript:") &&
        !lower.contains("onerror=") &&
        !lower.contains("onclick=") &&
        !lower.contains("onload=")
    }
}

/// Sanitize input by removing potentially dangerous characters
pub fn sanitize_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect::<String>()
        .trim()
        .to_string()
}

/// Sanitize and limit string length
pub fn sanitize_and_limit(input: &str, max_len: usize) -> String {
    let sanitized = sanitize_string(input);
    if sanitized.len() > max_len {
        sanitized.chars().take(max_len).collect()
    } else {
        sanitized
    }
}

/// Request size limit middleware factory
pub struct RequestSizeLimit {
    max_size: usize,
}

impl RequestSizeLimit {
    /// Create a new request size limiter
    /// Default is 1MB
    pub fn new(max_size: usize) -> Self {
        Self { max_size }
    }

    /// Default limit of 1MB
    pub fn default_limit() -> Self {
        Self::new(1024 * 1024)
    }

    /// Small limit of 64KB (for simple forms)
    pub fn small_limit() -> Self {
        Self::new(64 * 1024)
    }

    /// Large limit of 10MB (for file uploads)
    pub fn large_limit() -> Self {
        Self::new(10 * 1024 * 1024)
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestSizeLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Transform = RequestSizeLimitMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestSizeLimitMiddleware {
            service,
            max_size: self.max_size,
        }))
    }
}

pub struct RequestSizeLimitMiddleware<S> {
    service: S,
    max_size: usize,
}

impl<S, B> Service<ServiceRequest> for RequestSizeLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let content_length = req
            .headers()
            .get(actix_web::http::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        let max_size = self.max_size;

        if content_length > max_size {
            let response = HttpResponse::PayloadTooLarge()
                .json(serde_json::json!({
                    "error": "Request body too large",
                    "code": "PAYLOAD_TOO_LARGE",
                    "max_size": max_size
                }));

            return Box::pin(async move {
                Ok(req.into_response(response).map_into_right_body())
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_string() {
        assert_eq!(sanitize_string("  hello  "), "hello");
        assert_eq!(sanitize_string("hello\x00world"), "helloworld");
        assert_eq!(sanitize_string("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn test_sanitize_and_limit() {
        assert_eq!(sanitize_and_limit("hello world", 5), "hello");
        assert_eq!(sanitize_and_limit("hi", 10), "hi");
    }

    #[test]
    fn test_safe_path() {
        use patterns::is_safe_path;

        assert!(!is_safe_path("../etc/passwd"));
        assert!(!is_safe_path("/etc/passwd"));
        assert!(!is_safe_path("foo//bar"));
        assert!(is_safe_path("foo/bar/baz"));
    }

    #[test]
    fn test_xss_safe() {
        use patterns::is_xss_safe;

        assert!(!is_xss_safe("<script>alert(1)</script>"));
        assert!(!is_xss_safe("javascript:alert(1)"));
        assert!(!is_xss_safe("<img onerror=alert(1)>"));
        assert!(is_xss_safe("Hello, World!"));
    }
}
