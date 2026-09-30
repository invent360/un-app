//! Test server wrapper for real HTTP testing

use actix_web::{web, App};
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::oneshot;
use uno_app::server::{app::ServiceFactory, handlers::configure_api_routes};

/// Test server that wraps a real Actix HTTP server
pub struct TestServer {
    /// Base URL of the server (e.g., "http://127.0.0.1:8080")
    url: String,
    /// Shutdown signal sender
    _shutdown: oneshot::Sender<()>,
}

impl TestServer {
    /// Start a test server with the given service factory
    pub async fn start(factory: ServiceFactory) -> Self {
        // Bind to a random available port
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let url = format!("http://127.0.0.1:{}", port);

        let factory = Arc::new(factory);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        // Start server in background
        let server_url = url.clone();
        tokio::spawn(async move {
            let factory = factory.clone();
            let server = actix_web::HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new((*factory).clone()))
                    .configure(configure_api_routes)
            })
            .listen(listener)
            .expect("Failed to bind listener")
            .run();

            // Run until shutdown signal
            tokio::select! {
                _ = server => {}
                _ = shutdown_rx => {}
            }

            tracing::debug!("Test server at {} stopped", server_url);
        });

        // Give server time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Self {
            url,
            _shutdown: shutdown_tx,
        }
    }

    /// Get the base URL of the test server
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Build a full URL for a path
    pub fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.url, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[actix_web::test]
    #[ignore = "requires database connection"]
    async fn test_server_starts_and_responds() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL required");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&database_url)
            .await
            .expect("Database connection required");

        std::env::set_var("ADMIN_API_KEY", "test-key");
        std::env::set_var("ADMIN_CLIENT_ID", "test-admin");
        std::env::set_var("ADMIN_SECRET_KEY", "test-secret");

        let factory = ServiceFactory::new(pool);
        let server = TestServer::start(factory).await;

        // Test health endpoint
        let client = reqwest::Client::new();
        let response = client
            .get(server.build_url("/api/v1/health"))
            .send()
            .await
            .expect("Request should succeed");

        assert!(response.status().is_success());
    }
}
