//! Test harness for Phase 9 acceptance tests with real HTTP and PostgreSQL
//!
//! This module provides a reusable test infrastructure that runs against
//! actual database connections and HTTP endpoints, replacing mock-based tests.

#![cfg(feature = "ssr")]

mod client;
mod fixtures;
mod server;

pub use client::TestClient;
pub use fixtures::TestFixtures;
pub use server::TestServer;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tempfile::TempDir;
use uno_app::server::app::ServiceFactory;

/// Test harness with real database and HTTP server
pub struct TestHarness {
    /// PostgreSQL connection pool
    pub pool: PgPool,
    /// Actix test server
    pub server: TestServer,
    /// HTTP client with session management
    pub client: TestClient,
    /// Temporary directory for file storage
    pub temp_dir: TempDir,
    /// Test fixtures for database seeding
    pub fixtures: TestFixtures,
}

impl TestHarness {
    /// Create a new test harness
    ///
    /// # Panics
    /// Panics if DATABASE_URL is not set or database connection fails.
    /// This is intentional - missing database is a test failure, not a skip.
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL required for integration tests");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Database connection required for integration tests");

        // Create temp directory for file storage
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        std::env::set_var("FILE_STORAGE_LOCAL_PATH", temp_dir.path());

        // Set test API key
        std::env::set_var("ADMIN_API_KEY", "test-api-key-phase9");
        std::env::set_var("ADMIN_CLIENT_ID", "test-admin");
        std::env::set_var("ADMIN_SECRET_KEY", "test-secret-key-phase9");

        // F9: License issuance is controlled by --features test-issuance
        // Run tests with: cargo test --features ssr,test-issuance

        // F9: Configure session verification with test RSA keys
        let (public_keys, issuer, audience, _origin) = TestFixtures::test_session_config();
        std::env::set_var("UNO_SESSION_PUBLIC_KEYS", &public_keys);
        std::env::set_var("UNO_SESSION_ISSUER", &issuer);
        std::env::set_var("UNO_SESSION_AUDIENCE", &audience);

        // Set origin before creating factory (use localhost placeholder)
        std::env::set_var("UNO_PUBLIC_ORIGIN", "http://127.0.0.1:0");

        // Create service factory with real dependencies
        let factory = ServiceFactory::new(pool.clone());

        // Start test server
        let server = TestServer::start(factory).await;

        // Update origin with actual server URL
        std::env::set_var("UNO_PUBLIC_ORIGIN", server.url());

        // Create HTTP client
        let client = TestClient::new(server.url());

        // Create fixtures helper
        let fixtures = TestFixtures::new(pool.clone());

        Self {
            pool,
            server,
            client,
            temp_dir,
            fixtures,
        }
    }

    /// Create harness with custom pool configuration
    pub async fn with_max_connections(max_connections: u32) -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL required for integration tests");

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url)
            .await
            .expect("Database connection required for integration tests");

        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        std::env::set_var("FILE_STORAGE_LOCAL_PATH", temp_dir.path());
        std::env::set_var("ADMIN_API_KEY", "test-api-key-phase9");
        std::env::set_var("ADMIN_CLIENT_ID", "test-admin");
        std::env::set_var("ADMIN_SECRET_KEY", "test-secret-key-phase9");

        // F9: License issuance is controlled by --features test-issuance
        // Run tests with: cargo test --features ssr,test-issuance

        // F9: Configure session verification with test RSA keys
        let (public_keys, issuer, audience, _origin) = TestFixtures::test_session_config();
        std::env::set_var("UNO_SESSION_PUBLIC_KEYS", &public_keys);
        std::env::set_var("UNO_SESSION_ISSUER", &issuer);
        std::env::set_var("UNO_SESSION_AUDIENCE", &audience);
        std::env::set_var("UNO_PUBLIC_ORIGIN", "http://127.0.0.1:0");

        let factory = ServiceFactory::new(pool.clone());
        let server = TestServer::start(factory).await;

        // Update origin with actual server URL
        std::env::set_var("UNO_PUBLIC_ORIGIN", server.url());
        let client = TestClient::new(server.url());
        let fixtures = TestFixtures::new(pool.clone());

        Self {
            pool,
            server,
            client,
            temp_dir,
            fixtures,
        }
    }

    /// Get a reference to the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the server base URL
    pub fn url(&self) -> &str {
        self.server.url()
    }

    /// Clean up test data created during the test
    pub async fn cleanup(&self) {
        // Delete test licenses
        let _ = sqlx::query("DELETE FROM licenses WHERE lease_code LIKE 'TEST-%'")
            .execute(&self.pool)
            .await;

        // Delete test claims
        let _ = sqlx::query("DELETE FROM claims WHERE device_id LIKE 'test-device-%'")
            .execute(&self.pool)
            .await;

        // Delete test sessions
        let _ = sqlx::query("DELETE FROM sessions WHERE user_id LIKE 'test-user-%'")
            .execute(&self.pool)
            .await;

        // Delete test allocations
        let _ = sqlx::query("DELETE FROM allocations WHERE license_code LIKE 'TEST-%'")
            .execute(&self.pool)
            .await;
    }
}

/// Common test assertions
pub mod assertions {
    use reqwest::StatusCode;

    /// Assert response has expected status code
    pub fn assert_status(response: &reqwest::Response, expected: StatusCode) {
        assert_eq!(
            response.status(),
            expected,
            "Expected status {}, got {}",
            expected,
            response.status()
        );
    }

    /// Assert response is successful (2xx)
    pub fn assert_success(response: &reqwest::Response) {
        assert!(
            response.status().is_success(),
            "Expected success, got {}",
            response.status()
        );
    }

    /// Assert response is client error (4xx)
    pub fn assert_client_error(response: &reqwest::Response) {
        assert!(
            response.status().is_client_error(),
            "Expected client error, got {}",
            response.status()
        );
    }

    /// Assert response is server error (5xx)
    pub fn assert_server_error(response: &reqwest::Response) {
        assert!(
            response.status().is_server_error(),
            "Expected server error, got {}",
            response.status()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    #[ignore = "requires database connection"]
    async fn test_harness_creates_successfully() {
        let harness = TestHarness::new().await;

        // Verify pool is connected
        let result: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(harness.pool())
            .await
            .expect("Database should be connected");
        assert_eq!(result.0, 1);

        // Verify server is running
        let url = harness.url();
        assert!(url.starts_with("http://"));

        // Cleanup
        harness.cleanup().await;
    }
}
