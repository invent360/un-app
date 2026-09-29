//! Unity API configuration
//!
//! Configuration for connecting to the Unity API.
//!
//! SECURITY NOTE: JWT tokens and API secrets must NEVER be compiled into WASM builds.
//! For WASM/browser builds, authentication must be handled server-side via API calls.

/// Default Unity API base URL
pub const DEFAULT_BASE_URL: &str = "https://api.unityedge.io";

/// Default API key for Unity (publishable key - safe to expose)
pub const DEFAULT_API_KEY: &str = "sb_publishable_yKqi0fu5vV6G4ryUIMJuzw_NCoFEl1c";

// SECURITY FIX: Removed COMPILE_TIME_JWT_TOKEN
// JWT tokens must NEVER be embedded in browser builds via option_env!
// Use server-side token management instead.

/// Configuration for the Unity API client
#[derive(Debug, Clone)]
pub struct UnityApiConfig {
    pub base_url: String,
    pub api_key: String,
    pub jwt_token: String,
}

impl UnityApiConfig {
    /// Create a new configuration with the given API key and JWT token
    pub fn new(api_key: impl Into<String>, jwt_token: impl Into<String>) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: api_key.into(),
            jwt_token: jwt_token.into(),
        }
    }

    /// Create configuration from environment variables
    ///
    /// For WASM: Returns config WITHOUT JWT token - browser builds must not contain secrets.
    ///           Authenticated operations must be performed via server-side API calls.
    /// For SSR: Uses runtime environment variables (std::env::var)
    ///
    /// Reads:
    /// - `UNITY_API_URL` (optional, defaults to https://api.unityedge.io)
    /// - `UNITY_API_KEY` (optional, defaults to publishable key)
    /// - `UNITY_JWT_TOKEN` (SSR only - required for authenticated requests)
    #[cfg(target_arch = "wasm32")]
    pub fn from_env() -> Self {
        // SECURITY: WASM builds never include JWT tokens
        // All authenticated operations must go through server-side endpoints
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: DEFAULT_API_KEY.to_string(),
            jwt_token: String::new(), // No token in browser builds
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_env() -> Self {
        // API_TOKEN takes precedence over UNITY_JWT_TOKEN (inline override)
        let jwt_token = std::env::var("API_TOKEN")
            .or_else(|_| std::env::var("UNITY_JWT_TOKEN"))
            .unwrap_or_default();

        Self {
            base_url: std::env::var("UNITY_API_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string()),
            api_key: std::env::var("UNITY_API_KEY").unwrap_or_else(|_| DEFAULT_API_KEY.to_string()),
            jwt_token,
        }
    }

    /// Create configuration with just a JWT token (uses defaults for everything else)
    pub fn with_token(jwt_token: impl Into<String>) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: DEFAULT_API_KEY.to_string(),
            jwt_token: jwt_token.into(),
        }
    }

    /// Set a custom base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Check if JWT token is configured
    pub fn has_token(&self) -> bool {
        !self.jwt_token.is_empty()
    }
}

impl Default for UnityApiConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: DEFAULT_API_KEY.to_string(),
            jwt_token: String::new(),
        }
    }
}
