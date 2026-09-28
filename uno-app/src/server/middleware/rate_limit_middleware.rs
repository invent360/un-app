//! Async-safe rate limiting middleware
//!
//! Uses tokio::sync::RwLock for safe concurrent access under high load.
//! Implements sliding window rate limiting per client IP.

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::collections::HashMap;
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Rate limit configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub requests_per_window: u32,
    /// Window duration
    pub window_duration: Duration,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_window: 100,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300), // Cleanup every 5 minutes
        }
    }
}

impl RateLimitConfig {
    /// Create a strict rate limit (10 requests per minute)
    pub fn strict() -> Self {
        Self {
            requests_per_window: 10,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(120),
        }
    }

    /// Create a relaxed rate limit (1000 requests per minute)
    pub fn relaxed() -> Self {
        Self {
            requests_per_window: 1000,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(600),
        }
    }

    /// Create a custom rate limit
    pub fn custom(requests: u32, window_secs: u64) -> Self {
        Self {
            requests_per_window: requests,
            window_duration: Duration::from_secs(window_secs),
            cleanup_interval: Duration::from_secs(window_secs * 5),
        }
    }
}

/// Request tracking entry with sliding window support
#[derive(Clone)]
struct RequestEntry {
    /// Request timestamps within the current window
    timestamps: Vec<Instant>,
}

impl RequestEntry {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
        }
    }

    /// Add a request and return the count within the window
    fn add_request(&mut self, now: Instant, window: Duration) -> u32 {
        // Clean up old timestamps
        self.timestamps.retain(|&t| now.duration_since(t) <= window);
        self.timestamps.push(now);
        self.timestamps.len() as u32
    }

    /// Check if entry is stale (no requests in 2x window duration)
    fn is_stale(&self, now: Instant, window: Duration) -> bool {
        self.timestamps.is_empty() ||
            now.duration_since(*self.timestamps.last().unwrap()) > window * 2
    }
}

/// Async rate limiter state
pub struct RateLimiterState {
    requests: RwLock<HashMap<String, RequestEntry>>,
    last_cleanup: RwLock<Instant>,
}

impl RateLimiterState {
    fn new() -> Self {
        Self {
            requests: RwLock::new(HashMap::new()),
            last_cleanup: RwLock::new(Instant::now()),
        }
    }

    /// Check rate limit for a client
    async fn check_limit(&self, client_ip: &str, config: &RateLimitConfig) -> RateLimitResult {
        let now = Instant::now();

        // Perform cleanup if needed
        self.maybe_cleanup(now, config).await;

        let mut requests = self.requests.write().await;

        let entry = requests
            .entry(client_ip.to_string())
            .or_insert_with(RequestEntry::new);

        let count = entry.add_request(now, config.window_duration);

        if count > config.requests_per_window {
            let oldest = entry.timestamps.first().cloned();
            let retry_after = oldest
                .map(|t| config.window_duration.saturating_sub(now.duration_since(t)))
                .unwrap_or(config.window_duration);

            RateLimitResult::Limited {
                retry_after_secs: retry_after.as_secs(),
            }
        } else {
            RateLimitResult::Allowed {
                remaining: config.requests_per_window - count,
                reset_secs: config.window_duration.as_secs(),
            }
        }
    }

    /// Clean up stale entries periodically
    async fn maybe_cleanup(&self, now: Instant, config: &RateLimitConfig) {
        let should_cleanup = {
            let last = self.last_cleanup.read().await;
            now.duration_since(*last) > config.cleanup_interval
        };

        if should_cleanup {
            let mut last = self.last_cleanup.write().await;
            *last = now;
            drop(last);

            let mut requests = self.requests.write().await;
            requests.retain(|_, entry| !entry.is_stale(now, config.window_duration));
        }
    }
}

/// Result of rate limit check
enum RateLimitResult {
    Allowed {
        remaining: u32,
        reset_secs: u64,
    },
    Limited {
        retry_after_secs: u64,
    },
}

/// Rate limiter middleware factory
pub struct RateLimiter {
    config: RateLimitConfig,
    state: Arc<RateLimiterState>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: Arc::new(RateLimiterState::new()),
        }
    }

    pub fn default_config() -> Self {
        Self::new(RateLimitConfig::default())
    }

    /// Create a shared rate limiter state that can be reused
    pub fn with_shared_state(config: RateLimitConfig, state: Arc<RateLimiterState>) -> Self {
        Self { config, state }
    }

    /// Get the shared state for use with multiple limiters
    pub fn state(&self) -> Arc<RateLimiterState> {
        self.state.clone()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            config: self.config.clone(),
            state: self.state.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    config: RateLimitConfig,
    state: Arc<RateLimiterState>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        // Get client IP
        let client_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let state = self.state.clone();
        let config = self.config.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            match state.check_limit(&client_ip, &config).await {
                RateLimitResult::Limited { retry_after_secs } => {
                    let response = HttpResponse::TooManyRequests()
                        .insert_header(("Retry-After", retry_after_secs.to_string()))
                        .insert_header(("X-RateLimit-Limit", config.requests_per_window.to_string()))
                        .insert_header(("X-RateLimit-Remaining", "0"))
                        .json(serde_json::json!({
                            "error": "Rate limit exceeded",
                            "retry_after": retry_after_secs,
                            "code": "RATE_LIMITED"
                        }));

                    // We need to reconstruct the request to use into_response
                    // Since we've already moved it into fut, we create a new response
                    let (req, _) = fut.await?.into_parts();
                    Ok(ServiceResponse::new(req, response).map_into_right_body())
                }
                RateLimitResult::Allowed { remaining, reset_secs } => {
                    let mut res = fut.await?;

                    // Add rate limit headers
                    res.headers_mut().insert(
                        actix_web::http::header::HeaderName::from_static("x-ratelimit-limit"),
                        config.requests_per_window.to_string().parse().unwrap(),
                    );
                    res.headers_mut().insert(
                        actix_web::http::header::HeaderName::from_static("x-ratelimit-remaining"),
                        remaining.to_string().parse().unwrap(),
                    );
                    res.headers_mut().insert(
                        actix_web::http::header::HeaderName::from_static("x-ratelimit-reset"),
                        reset_secs.to_string().parse().unwrap(),
                    );

                    Ok(res.map_into_left_body())
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_allows_under_limit() {
        let state = RateLimiterState::new();
        let config = RateLimitConfig::custom(5, 60);

        for _ in 0..5 {
            let result = state.check_limit("test_ip", &config).await;
            assert!(matches!(result, RateLimitResult::Allowed { .. }));
        }
    }

    #[tokio::test]
    async fn test_rate_limit_blocks_over_limit() {
        let state = RateLimiterState::new();
        let config = RateLimitConfig::custom(3, 60);

        // First 3 should pass
        for _ in 0..3 {
            let result = state.check_limit("test_ip", &config).await;
            assert!(matches!(result, RateLimitResult::Allowed { .. }));
        }

        // 4th should be limited
        let result = state.check_limit("test_ip", &config).await;
        assert!(matches!(result, RateLimitResult::Limited { .. }));
    }

    #[tokio::test]
    async fn test_rate_limit_separate_ips() {
        let state = RateLimiterState::new();
        let config = RateLimitConfig::custom(2, 60);

        // IP 1
        state.check_limit("ip1", &config).await;
        state.check_limit("ip1", &config).await;

        // IP 1 should be at limit
        let result = state.check_limit("ip1", &config).await;
        assert!(matches!(result, RateLimitResult::Limited { .. }));

        // IP 2 should still be allowed
        let result = state.check_limit("ip2", &config).await;
        assert!(matches!(result, RateLimitResult::Allowed { .. }));
    }
}
