//! Shared HTTP client with configured timeouts
//!
//! Provides a global HTTP client with request and connection timeouts
//! to prevent sync jobs from hanging indefinitely on external API calls.

use std::sync::OnceLock;
use std::time::Duration;

/// Default request timeout in seconds (30 seconds)
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default connection timeout in seconds (10 seconds)
pub const CONNECT_TIMEOUT_SECS: u64 = 10;

/// Global HTTP client instance with configured timeouts
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

/// Get the shared HTTP client with timeout configured
///
/// This client is lazily initialized on first use and shared across all callers.
/// Configuration:
/// - Request timeout: 30 seconds (total time for request/response)
/// - Connection timeout: 10 seconds (time to establish connection)
///
/// # Example
/// ```ignore
/// let client = crate::api::http_client::get_client();
/// let response = client.get("https://api.example.com").send().await?;
/// ```
pub fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
            .build()
            .expect("Failed to build HTTP client")
    })
}

/// Create a client with custom timeout (for specific use cases)
///
/// Use this when you need a different timeout than the default 30 seconds.
/// Note: This creates a new client each time - prefer `get_client()` for
/// standard timeouts to reuse the shared connection pool.
pub fn client_with_timeout(timeout_secs: u64) -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .build()
        .expect("Failed to build HTTP client")
}
