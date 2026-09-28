//! Security headers middleware
//!
//! This middleware adds comprehensive security headers to all responses,
//! following OWASP security best practices.

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::future::{ready, Ready, Future};
use std::pin::Pin;

/// Content Security Policy configuration
/// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP
const CSP_POLICY: &str = concat!(
    "default-src 'self'; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "style-src 'self' 'unsafe-inline'; ",
    "img-src 'self' data: https:; ",
    "font-src 'self' data:; ",
    "connect-src 'self' https:; ",
    "frame-ancestors 'none'; ",
    "base-uri 'self'; ",
    "form-action 'self'; ",
    "upgrade-insecure-requests"
);

/// HSTS configuration
/// max-age=31536000 (1 year), includeSubDomains, preload
/// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security
const HSTS_VALUE: &str = "max-age=31536000; includeSubDomains; preload";

/// Permissions Policy (formerly Feature-Policy)
/// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Permissions-Policy
const PERMISSIONS_POLICY: &str = concat!(
    "accelerometer=(), ",
    "camera=(), ",
    "geolocation=(), ",
    "gyroscope=(), ",
    "magnetometer=(), ",
    "microphone=(), ",
    "payment=(), ",
    "usb=()"
);

/// Security headers middleware factory
pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SecurityHeadersMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersMiddleware { service }))
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // Add security headers
            let headers = res.headers_mut();

            // Content-Type Options - Prevent MIME type sniffing
            headers.insert(
                actix_web::http::header::X_CONTENT_TYPE_OPTIONS,
                "nosniff".parse().unwrap(),
            );

            // Frame Options - Prevent clickjacking
            headers.insert(
                actix_web::http::header::X_FRAME_OPTIONS,
                "DENY".parse().unwrap(),
            );

            // XSS Protection - Legacy browser XSS protection
            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-xss-protection"),
                "1; mode=block".parse().unwrap(),
            );

            // Referrer Policy - Control referrer information
            headers.insert(
                actix_web::http::header::HeaderName::from_static("referrer-policy"),
                "strict-origin-when-cross-origin".parse().unwrap(),
            );

            // Content Security Policy - Prevent XSS and data injection
            headers.insert(
                actix_web::http::header::HeaderName::from_static("content-security-policy"),
                CSP_POLICY.parse().unwrap(),
            );

            // HTTP Strict Transport Security - Force HTTPS
            headers.insert(
                actix_web::http::header::HeaderName::from_static("strict-transport-security"),
                HSTS_VALUE.parse().unwrap(),
            );

            // Permissions Policy - Restrict browser features
            headers.insert(
                actix_web::http::header::HeaderName::from_static("permissions-policy"),
                PERMISSIONS_POLICY.parse().unwrap(),
            );

            // Cross-Origin Embedder Policy
            headers.insert(
                actix_web::http::header::HeaderName::from_static("cross-origin-embedder-policy"),
                "require-corp".parse().unwrap(),
            );

            // Cross-Origin Opener Policy
            headers.insert(
                actix_web::http::header::HeaderName::from_static("cross-origin-opener-policy"),
                "same-origin".parse().unwrap(),
            );

            // Cross-Origin Resource Policy
            headers.insert(
                actix_web::http::header::HeaderName::from_static("cross-origin-resource-policy"),
                "same-origin".parse().unwrap(),
            );

            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csp_policy_is_valid() {
        // Ensure CSP policy contains required directives
        assert!(CSP_POLICY.contains("default-src"));
        assert!(CSP_POLICY.contains("script-src"));
        assert!(CSP_POLICY.contains("frame-ancestors 'none'"));
        assert!(CSP_POLICY.contains("upgrade-insecure-requests"));
    }

    #[test]
    fn test_hsts_value_is_valid() {
        // Ensure HSTS has minimum 1 year max-age
        assert!(HSTS_VALUE.contains("max-age=31536000"));
        assert!(HSTS_VALUE.contains("includeSubDomains"));
    }

    #[test]
    fn test_permissions_policy_disables_sensitive_apis() {
        // Ensure sensitive APIs are disabled
        assert!(PERMISSIONS_POLICY.contains("camera=()"));
        assert!(PERMISSIONS_POLICY.contains("microphone=()"));
        assert!(PERMISSIONS_POLICY.contains("geolocation=()"));
    }
}
