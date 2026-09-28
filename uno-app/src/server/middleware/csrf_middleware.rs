//! CSRF (Cross-Site Request Forgery) protection middleware
//!
//! Implements double-submit cookie pattern for CSRF protection:
//! 1. Server sets a CSRF token in a cookie
//! 2. Client must include the same token in X-CSRF-Token header
//! 3. Server validates that cookie and header values match
//!
//! Protected methods: POST, PUT, PATCH, DELETE

use actix_web::{
    cookie::{Cookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{header::HeaderName, Method},
    Error, HttpResponse,
};
use rand::Rng;
use sha2::{Sha256, Digest};
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

/// CSRF token cookie name
const CSRF_COOKIE_NAME: &str = "csrf_token";

/// CSRF header name
const CSRF_HEADER_NAME: &str = "x-csrf-token";

/// Token validity in seconds (1 hour)
const TOKEN_VALIDITY_SECS: u64 = 3600;

/// CSRF protection middleware factory
pub struct CsrfProtection {
    /// Secret key for token generation (should be from secrets manager in prod)
    secret_key: String,
    /// Whether to skip CSRF for API routes (if using separate auth)
    skip_api_routes: bool,
}

impl CsrfProtection {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
            skip_api_routes: false,
        }
    }

    /// Skip CSRF validation for /api routes (if using token auth)
    pub fn skip_api(mut self, skip: bool) -> Self {
        self.skip_api_routes = skip;
        self
    }

    /// Generate a new CSRF token (used in tests)
    #[allow(dead_code)]
    fn generate_token(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let random_bytes: [u8; 32] = rand::rng().random();
        let random_hex = hex::encode(random_bytes);

        // Create token: timestamp.random.signature
        let payload = format!("{}.{}", timestamp, random_hex);
        let signature = self.sign(&payload);

        format!("{}.{}", payload, signature)
    }

    /// Sign a payload with the secret key (used in tests)
    #[allow(dead_code)]
    fn sign(&self, payload: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        hasher.update(self.secret_key.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Validate a CSRF token (used in tests)
    #[allow(dead_code)]
    fn validate_token(&self, token: &str) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        let timestamp: u64 = match parts[0].parse() {
            Ok(t) => t,
            Err(_) => return false,
        };

        // Check token age
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now - timestamp > TOKEN_VALIDITY_SECS {
            return false;
        }

        // Verify signature
        let payload = format!("{}.{}", parts[0], parts[1]);
        let expected_signature = self.sign(&payload);

        constant_time_compare(&expected_signature, parts[2])
    }
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

impl<S, B> Transform<S, ServiceRequest> for CsrfProtection
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Transform = CsrfMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfMiddleware {
            service,
            secret_key: self.secret_key.clone(),
            skip_api_routes: self.skip_api_routes,
        }))
    }
}

pub struct CsrfMiddleware<S> {
    service: S,
    secret_key: String,
    skip_api_routes: bool,
}

impl<S> CsrfMiddleware<S> {
    fn generate_token(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let random_bytes: [u8; 32] = rand::rng().random();
        let random_hex = hex::encode(random_bytes);

        let payload = format!("{}.{}", timestamp, random_hex);
        let signature = self.sign(&payload);

        format!("{}.{}", payload, signature)
    }

    fn sign(&self, payload: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        hasher.update(self.secret_key.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn validate_token(&self, token: &str) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        let timestamp: u64 = match parts[0].parse() {
            Ok(t) => t,
            Err(_) => return false,
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now - timestamp > TOKEN_VALIDITY_SECS {
            return false;
        }

        let payload = format!("{}.{}", parts[0], parts[1]);
        let expected_signature = self.sign(&payload);

        constant_time_compare(&expected_signature, parts[2])
    }
}

impl<S, B> Service<ServiceRequest> for CsrfMiddleware<S>
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
        let method = req.method().clone();
        let path = req.path().to_string();

        // Skip CSRF for safe methods (GET, HEAD, OPTIONS)
        let is_safe_method = matches!(method, Method::GET | Method::HEAD | Method::OPTIONS);

        // Skip API routes if configured
        let is_api_route = path.starts_with("/api/");
        let skip_validation = is_safe_method || (self.skip_api_routes && is_api_route);

        if skip_validation {
            // For safe methods, ensure CSRF cookie is set
            let token = self.generate_token();
            let fut = self.service.call(req);

            return Box::pin(async move {
                let mut res = fut.await?;

                // Set CSRF cookie on GET requests to ensure token is available
                if method == Method::GET {
                    let cookie = Cookie::build(CSRF_COOKIE_NAME, token)
                        .path("/")
                        .same_site(SameSite::Strict)
                        .http_only(false) // Must be readable by JS
                        .secure(true)
                        .finish();

                    if let Ok(cookie_val) = cookie.to_string().parse() {
                        res.headers_mut().insert(
                            actix_web::http::header::SET_COOKIE,
                            cookie_val,
                        );
                    }
                }

                Ok(res.map_into_left_body())
            });
        }

        // For state-changing methods, validate CSRF token
        let cookie_token = req
            .cookie(CSRF_COOKIE_NAME)
            .map(|c| c.value().to_string());

        let header_token = req
            .headers()
            .get(HeaderName::from_static(CSRF_HEADER_NAME))
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Validate tokens
        let valid = match (&cookie_token, &header_token) {
            (Some(cookie), Some(header)) => {
                // Both must match and be valid
                cookie == header && self.validate_token(cookie)
            }
            _ => false,
        };

        if !valid {
            let response = HttpResponse::Forbidden()
                .json(serde_json::json!({
                    "error": "CSRF validation failed",
                    "code": "CSRF_INVALID"
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
    fn test_token_generation_and_validation() {
        let csrf = CsrfProtection::new("test_secret_key_12345");
        let token = csrf.generate_token();

        assert!(csrf.validate_token(&token));
    }

    #[test]
    fn test_invalid_token_rejected() {
        let csrf = CsrfProtection::new("test_secret_key_12345");

        assert!(!csrf.validate_token("invalid.token.here"));
        assert!(!csrf.validate_token(""));
        assert!(!csrf.validate_token("no_dots"));
    }

    #[test]
    fn test_tampered_token_rejected() {
        let csrf = CsrfProtection::new("test_secret_key_12345");
        let token = csrf.generate_token();

        // Tamper with the token
        let parts: Vec<&str> = token.split('.').collect();
        let tampered = format!("{}.tampered.{}", parts[0], parts[2]);

        assert!(!csrf.validate_token(&tampered));
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare("hello", "hello"));
        assert!(!constant_time_compare("hello", "world"));
        assert!(!constant_time_compare("hello", "hell"));
    }
}
